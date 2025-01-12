// use anyhow::Result;
// use proc_macro2::TokenStream;
// use quote::{format_ident, quote, ToTokens};
// use std::{
//     collections::{HashMap, HashSet},
//     fmt,
// };
// use syn::{GenericArgument, Item, ItemImpl, Path, PathArguments, Type, TypePath};
// use syn_utils::{get_arguments_of_path, get_element_of_path, get_element_of_ty};

// mod syn_utils;

// pub fn build_transitivity(files: &[&str]) -> Result<TokenStream> {
//     let mut asts = Vec::new();
//     for file in files {
//         let s = std::fs::read_to_string(file)?;
//         let ast: syn::File = syn::parse_str(&s)?;
//         asts.push(ast);
//     }
//     build_transitivity_raw(&asts)
// }
// fn build_transitivity_raw(files: &[syn::File]) -> Result<TokenStream> {
//     let mut registry = Registry::new();

//     // ファイル内で定義された型を収集
//     for file in files {
//         registry.collect_defined_types(file);
//     }

//     // 既存のFrom実装を収集
//     for file in files {
//         registry.collect_from_impls(file);
//     }

//     // print defined_types
//     for defined_type in &registry.defined_types {
//         println!("defined_type: {}", defined_type);
//     }

//     // print converts
//     for convert in &registry.converts {
//         println!("convert: {}", convert);
//     }

//     // 推移的な変換を生成
//     let ts = registry.generate_transitive_conversions()?;
//     Ok(quote! {
//         use crate::schema::*;
//         #ts
//     })
// }
// struct Registry {
//     defined_types: HashSet<Type>,
//     converts: HashSet<(Type, Type)>,
//     converts_generated: Vec<(Type, Type)>,
//     converts_from: HashMap<Type, Vec<Type>>,
// }
// impl Registry {
//     fn new() -> Self {
//         Self {
//             defined_types: HashSet::new(),
//             converts: HashSet::new(),
//             converts_generated: Vec::new(),
//             converts_from: HashMap::new(),
//         }
//     }
//     fn collect_defined_types(&mut self, file: &syn::File) {
//         for item in &file.items {
//             match item {
//                 Item::Struct(item_struct) => {
//                     self.defined_types.insert(to_ty(&item_struct.ident));
//                 }
//                 Item::Enum(item_enum) => {
//                     self.defined_types.insert(to_ty(&item_enum.ident));
//                 }
//                 _ => {}
//             }
//         }
//     }
//     fn collect_from_impls(&mut self, file: &syn::File) {
//         for item in &file.items {
//             if let Item::Impl(item_impl) = item {
//                 self.collect_from_impl(item_impl);
//             }
//         }
//     }
//     fn collect_from_impl(&mut self, item_impl: &syn::ItemImpl) {
//         item_impl.

//         if let Some(path) = get_convert_from_element(&item_impl.trait_) {

//         }

//     }
// }

// fn to_ty(ident: &syn::Ident) -> Type {
//     syn::parse_quote!(#ident)
// }
// fn get_convert_from_element(path: &Path) -> Option<&Type> {
//     get_element_of_path(path, &[&["std", "convert"], &["core", "convert"]], "From")
// }
