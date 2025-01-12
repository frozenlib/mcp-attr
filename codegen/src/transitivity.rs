#![allow(unused)]
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::{
    collections::{HashMap, HashSet},
    fmt,
};
use syn::{GenericArgument, Item, ItemImpl, Path, PathArguments, Type, TypePath};

pub fn build_transitivity(files: &[&str]) -> Result<TokenStream> {
    let mut asts = Vec::new();
    for file in files {
        let s = std::fs::read_to_string(file)?;
        let ast: syn::File = syn::parse_str(&s)?;
        asts.push(ast);
    }
    build_transitivity_raw(&asts)
}

fn build_transitivity_raw(files: &[syn::File]) -> Result<TokenStream> {
    let mut registry = Registry::new();

    // ファイル内で定義された型を収集
    for file in files {
        registry.collect_defined_types(file);
    }

    // 既存のFrom実装を収集
    for file in files {
        registry.collect_from_impls(file);
    }

    // print defined_types
    for defined_type in &registry.defined_types {
        println!("defined_type: {}", defined_type);
    }

    // print converts
    for convert in &registry.converts {
        println!("convert: {}", convert);
    }

    // 推移的な変換を生成
    let ts = registry.generate_transitive_conversions()?;
    Ok(quote! {
        use crate::schema::*;
        #ts
    })
}

struct Registry {
    defined_types: HashSet<String>,
    converts: Vec<Convert>,
    generated_converts: HashSet<(String, String)>,
}

impl Registry {
    fn new() -> Self {
        Self {
            defined_types: HashSet::new(),
            converts: vec![],
            generated_converts: HashSet::new(),
        }
    }

    // ファイル内で定義された型を収集
    fn collect_defined_types(&mut self, file: &syn::File) {
        for item in &file.items {
            if let Item::Struct(item_struct) = item {
                self.defined_types.insert(item_struct.ident.to_string());
            } else if let Item::Enum(item_enum) = item {
                self.defined_types.insert(item_enum.ident.to_string());
            }
        }
    }

    // 既存のFrom実装を収集
    fn collect_from_impls(&mut self, file: &syn::File) {
        for item in &file.items {
            if let Item::Impl(item_impl) = item {
                if let Some((_, trait_path, _)) = &item_impl.trait_ {
                    if is_from_trait(trait_path) {
                        if let Some(convert) = self.extract_from_impl(item_impl) {
                            self.converts.push(convert);
                        }
                    }
                }
            }
        }
    }

    // From<T> for U の実装から変換情報を抽出
    fn extract_from_impl(&self, item_impl: &ItemImpl) -> Option<Convert> {
        // 変換先の型を取得
        let to_type = &item_impl.self_ty;

        // From<T>のTを取得
        if let Some((_, trait_path, _)) = &item_impl.trait_ {
            if let Some(last_segment) = trait_path.segments.last() {
                if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    if let Some(GenericArgument::Type(from_type)) = args.args.first() {
                        // Self型を使用した変換をチェック
                        if let Some(from_str) = Self::type_to_string(from_type) {
                            if let Some(to_str) = Self::type_to_string(to_type) {
                                // 変換元と変換先が同じ型の場合はスキップ（Self型の問題を回避）
                                if from_str == to_str {
                                    return None;
                                }

                                // Self型が含まれている場合はスキップ
                                if from_str.contains("Self") || to_str.contains("Self") {
                                    return None;
                                }
                            }
                        }

                        return Some(Convert {
                            from: from_type.clone(),
                            to: (**to_type).clone(),
                        });
                    }
                }
            }
        }

        None
    }

    // 型名を文字列として取得（ジェネリックパラメータを含む）
    fn type_to_string(ty: &Type) -> Option<String> {
        match ty {
            Type::Path(TypePath { path, .. }) => {
                // 型のパスを文字列として取得
                let mut result = String::new();

                // パスのセグメントを処理
                for (i, segment) in path.segments.iter().enumerate() {
                    if i > 0 {
                        result.push_str("::");
                    }

                    // セグメント名を追加
                    result.push_str(&segment.ident.to_string());

                    // ジェネリックパラメータを処理
                    match &segment.arguments {
                        PathArguments::None => {}
                        PathArguments::AngleBracketed(args) => {
                            result.push('<');

                            for (j, arg) in args.args.iter().enumerate() {
                                if j > 0 {
                                    result.push_str(", ");
                                }

                                match arg {
                                    GenericArgument::Type(arg_type) => {
                                        // 再帰的に型を文字列化
                                        if let Some(type_str) = Self::type_to_string(arg_type) {
                                            result.push_str(&type_str);
                                        } else {
                                            // 型を文字列化できない場合は、トークンストリームを使用
                                            result
                                                .push_str(&arg_type.to_token_stream().to_string());
                                        }
                                    }
                                    // その他のジェネリック引数（ライフタイム、定数など）
                                    _ => {
                                        result.push_str(&arg.to_token_stream().to_string());
                                    }
                                }
                            }

                            result.push('>');
                        }
                        PathArguments::Parenthesized(args) => {
                            // 関数ポインタなどの場合
                            result.push_str(&args.to_token_stream().to_string());
                        }
                    }
                }

                Some(result)
            }
            // その他の型（参照型、配列型など）
            _ => Some(ty.to_token_stream().to_string()),
        }
    }

    // 推移的な変換を生成
    fn generate_transitive_conversions(&mut self) -> Result<TokenStream> {
        let mut result = TokenStream::new();
        let mut new_converts = Vec::new();

        // 既存の変換関係をマップに変換
        let mut conversion_map: HashMap<String, Vec<String>> = HashMap::new();
        for convert in &self.converts {
            if let (Some(from), Some(to)) = (
                Self::type_to_string(&convert.from),
                Self::type_to_string(&convert.to),
            ) {
                // 変換元と変換先が同じ型の場合はスキップ（Self型の問題を回避）
                if from == to {
                    continue;
                }

                conversion_map
                    .entry(from.clone())
                    .or_default()
                    .push(to.clone());

                // 既存の変換を記録
                self.generated_converts.insert((from, to));
            }
        }

        // 推移的な変換を見つける
        let mut found_new = true;
        while found_new {
            found_new = false;
            let current_map = conversion_map.clone();

            for (from, to_list) in &current_map {
                for to in to_list {
                    if let Some(next_to_list) = current_map.get(to) {
                        for next_to in next_to_list {
                            // 自己変換は除外
                            if from == next_to {
                                continue;
                            }

                            // 既に生成済みの変換は除外
                            if self
                                .generated_converts
                                .contains(&(from.clone(), next_to.clone()))
                            {
                                continue;
                            }

                            // 変換先が定義された型の場合のみ実装
                            // ジェネリック型の場合は、ベース型名（ジェネリックパラメータを除いた部分）で確認
                            let base_type = extract_base_type(next_to);
                            if self.defined_types.contains(&base_type) {
                                // 新しい変換を追加
                                self.generated_converts
                                    .insert((from.clone(), next_to.clone()));
                                conversion_map
                                    .entry(from.clone())
                                    .or_default()
                                    .push(next_to.clone());

                                // 型文字列からSyn::Type構造体を生成
                                let from_type = parse_type_from_string(from);
                                let to_type = parse_type_from_string(next_to);

                                new_converts.push((
                                    from_type,
                                    to_type,
                                    from.clone(),
                                    to.clone(),
                                    next_to.clone(),
                                ));
                                found_new = true;
                            }
                        }
                    }
                }
            }
        }

        // 生成した変換をソート
        new_converts.sort_by(|(_, _, from_a, _, to_a), (_, _, from_b, _, to_b)| {
            // まず変換元の型でソート
            let from_cmp = from_a.cmp(from_b);
            if from_cmp != std::cmp::Ordering::Equal {
                return from_cmp;
            }
            // 変換元が同じ場合は変換先でソート
            to_a.cmp(to_b)
        });

        // 生成した変換を実装
        for (from_type, to_type, _from_str, via_str, _to_str) in new_converts {
            // 中間型を文字列からパース
            let via_type = parse_type_from_string(&via_str);

            let impl_code = quote! {
                impl From<#from_type> for #to_type {
                    fn from(value: #from_type) -> Self {
                        let intermediate: #via_type = value.into();
                        intermediate.into()
                    }
                }
            };
            result.extend(impl_code);
        }

        Ok(result)
    }
}

// 型名からベース型（ジェネリックパラメータを除いた部分）を抽出
fn extract_base_type(type_str: &str) -> String {
    // ジェネリックパラメータの開始位置を見つける
    if let Some(pos) = type_str.find('<') {
        type_str[0..pos].to_string()
    } else {
        type_str.to_string()
    }
}

// 文字列からSyn::Type構造体を生成
fn parse_type_from_string(type_str: &str) -> Type {
    // 文字列をパースしてType構造体を生成
    match syn::parse_str::<Type>(type_str) {
        Ok(ty) => ty,
        Err(_) => {
            // パースに失敗した場合は、単純な識別子として扱う
            let ident = format_ident!("{}", type_str);
            Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: syn::punctuated::Punctuated::from_iter(vec![syn::PathSegment {
                        ident,
                        arguments: PathArguments::None,
                    }]),
                },
            })
        }
    }
}

#[derive(Debug)]
struct Convert {
    from: Type,
    to: Type,
}
impl fmt::Display for Convert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} -> {}",
            self.from.to_token_stream(),
            self.to.to_token_stream()
        )
    }
}

// パスがFromトレイトを指しているかチェック
fn is_from_trait(path: &Path) -> bool {
    // パスの最後のセグメントが"From"かどうかをチェック
    if let Some(last_segment) = path.segments.last() {
        return last_segment.ident == "From";
    }

    false
}
