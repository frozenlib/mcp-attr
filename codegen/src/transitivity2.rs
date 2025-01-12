use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::{GenericArgument, Item, Path, PathArguments, Type, TypePath};

#[derive(Debug)]
struct Registry {
    defined_types: HashSet<Type>,
    converts: Vec<Convert>,
    generated_converts: HashSet<(Type, Type)>,
}

#[derive(Debug)]
struct Convert {
    from: Type,
    to: Type,
}

impl Registry {
    fn new() -> Self {
        Self {
            defined_types: HashSet::new(),
            converts: Vec::new(),
            generated_converts: HashSet::new(),
        }
    }
}

pub fn build_transitivity(files: &[&str]) -> Result<TokenStream> {
    let mut asts = Vec::new();
    for file in files {
        let s = std::fs::read_to_string(file)?;
        let ast: syn::File = syn::parse_str(&s)?;
        asts.push(ast);
    }
    let ts = build_transitivity_raw(&asts)?;
    Ok(quote::quote! {
        use crate::schema::*;
        use crate::utils::*;
        #ts
    })
}

fn build_transitivity_raw(files: &[syn::File]) -> Result<TokenStream> {
    let mut registry = Registry::new();

    // 1. 型定義の収集
    for file in files {
        collect_defined_types(file, &mut registry)?;
    }

    // 2. 既存の型変換の収集
    for file in files {
        collect_converts(file, &mut registry)?;
    }

    // 3. 推移的な型変換の生成
    let generated_impls = generate_transitive_converts(&mut registry)?;

    Ok(quote! {
        #(#generated_impls)*
    })
}

fn collect_defined_types(file: &syn::File, registry: &mut Registry) -> Result<()> {
    for item in &file.items {
        match item {
            Item::Struct(item_struct) => {
                let ty = Type::Path(TypePath {
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: syn::punctuated::Punctuated::from_iter(vec![syn::PathSegment {
                            ident: item_struct.ident.clone(),
                            arguments: PathArguments::None,
                        }]),
                    },
                });
                registry.defined_types.insert(ty);
            }
            Item::Enum(item_enum) => {
                let ty = Type::Path(TypePath {
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: syn::punctuated::Punctuated::from_iter(vec![syn::PathSegment {
                            ident: item_enum.ident.clone(),
                            arguments: PathArguments::None,
                        }]),
                    },
                });
                registry.defined_types.insert(ty);
            }
            _ => {}
        }
    }
    Ok(())
}

fn collect_converts(file: &syn::File, registry: &mut Registry) -> Result<()> {
    for item in &file.items {
        if let Item::Impl(item_impl) = item {
            if let Some((_, trait_path, _)) = &item_impl.trait_ {
                if trait_path
                    .segments
                    .last()
                    .map(|s| s.ident == "From")
                    .unwrap_or(false)
                {
                    if let Some(segment) = trait_path.segments.last() {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(from_type)) = args.args.first() {
                                let to_type = &*item_impl.self_ty;

                                // ジェネリック型の場合はスキップ
                                if has_generic_params(to_type) {
                                    continue;
                                }

                                let normalized_from = normalize_type(from_type, to_type)?;
                                let normalized_to = normalize_type(to_type, to_type)?;

                                // 既存の変換として登録
                                registry.converts.push(Convert {
                                    from: normalized_from.clone(),
                                    to: normalized_to.clone(),
                                });

                                // 既に生成済みとしてマーク
                                registry
                                    .generated_converts
                                    .insert((normalized_from, normalized_to));
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn has_generic_params(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => type_path
            .path
            .segments
            .iter()
            .any(|segment| matches!(segment.arguments, PathArguments::AngleBracketed(_))),
        _ => false,
    }
}

fn normalize_type(ty: &Type, self_type: &Type) -> Result<Type> {
    match ty {
        Type::Path(type_path) => {
            let mut normalized = type_path.clone();
            if let Some(segment) = normalized.path.segments.first_mut() {
                if segment.ident == "Self" {
                    return Ok(self_type.clone());
                }
            }
            Ok(Type::Path(normalized))
        }
        Type::Reference(type_ref) => {
            let elem = normalize_type(&type_ref.elem, self_type)?;
            Ok(Type::Reference(syn::TypeReference {
                elem: Box::new(elem),
                ..type_ref.clone()
            }))
        }
        _ => Ok(ty.clone()),
    }
}

fn generate_transitive_converts(registry: &mut Registry) -> Result<Vec<TokenStream>> {
    let mut conversion_map: HashMap<Type, Vec<Type>> = HashMap::new();
    let mut generated_impls = Vec::new();

    // 既存の変換からマップを構築
    for convert in &registry.converts {
        conversion_map
            .entry(convert.from.clone())
            .or_default()
            .push(convert.to.clone());
    }

    // 到達可能な型のマップを構築
    let mut reachable: HashMap<Type, HashSet<(Type, Vec<Type>)>> = HashMap::new();
    for (from_type, to_types) in &conversion_map {
        for to_type in to_types {
            // 同じ型への変換はスキップ
            if from_type == to_type {
                continue;
            }

            reachable
                .entry(from_type.clone())
                .or_default()
                .insert((to_type.clone(), vec![to_type.clone()]));
        }
    }

    let mut new_converts = true;
    while new_converts {
        new_converts = false;
        let mut new_reachable = Vec::new();

        // 新しい到達可能な型を探す
        for (from_type, to_set) in &reachable {
            for (to_type, path) in to_set {
                if let Some(next_types) = conversion_map.get(to_type) {
                    for next_type in next_types {
                        // 同じ型への変換はスキップ
                        if from_type == next_type {
                            continue;
                        }

                        let mut new_path = path.clone();
                        new_path.push(next_type.clone());

                        // 既に到達可能な型の場合はスキップ
                        if let Some(existing_paths) = reachable.get(from_type) {
                            if existing_paths.iter().any(|(t, _)| t == next_type) {
                                continue;
                            }
                        }

                        new_reachable.push((from_type.clone(), next_type.clone(), new_path));
                        new_converts = true;
                    }
                }
            }
        }

        // 新しい到達可能な型を追加
        for (from_type, to_type, path) in new_reachable {
            reachable
                .entry(from_type.clone())
                .or_default()
                .insert((to_type, path));
        }
    }

    // 推移的な変換を生成
    for (from_type, to_set) in &reachable {
        for (to_type, path) in to_set {
            // 直接の変換はスキップ
            if path.len() == 1 {
                continue;
            }

            // 複数のパスがある場合はスキップ
            let paths_to_target = to_set.iter().filter(|(t, _)| t == to_type).count();
            if paths_to_target > 1 {
                continue;
            }

            // 外部クレートの型への変換はスキップ
            if !registry.defined_types.contains(to_type) {
                continue;
            }

            // 変換を生成
            let through_type = &path[path.len() - 2];
            let impl_tokens = generate_conversion_impl(from_type, through_type, to_type);
            generated_impls.push(impl_tokens);
        }
    }

    Ok(generated_impls)
}

fn generate_conversion_impl(from_type: &Type, through_type: &Type, to_type: &Type) -> TokenStream {
    // 推移的な変換のみを生成
    quote! {
        impl ::std::convert::From<#from_type> for #to_type {
            fn from(value: #from_type) -> Self {
                <#to_type as ::std::convert::From<#through_type>>::from(
                    <#through_type as ::std::convert::From<#from_type>>::from(value)
                )
            }
        }
    }
}

#[cfg(test)]
mod tests;
