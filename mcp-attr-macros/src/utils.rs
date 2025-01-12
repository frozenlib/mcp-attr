#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use structmeta::{NameArgs, NameValue, StructMeta};
use syn::{
    Attribute, Expr, FnArg, Ident, ImplItem, ImplItemFn, ItemFn, ItemImpl, Lit, LitStr, Meta, Pat,
    PatLit, PatType, Path, Result, Type, parse::Parse, parse2, spanned::Spanned,
};
use uri_template_ex::UriTemplate;

use crate::{
    ItemAttr,
    syn_utils::{get_element, is_path, is_type},
};

pub(crate) fn build_if(
    cond: bool,
    then: impl FnOnce() -> Result<TokenStream>,
) -> Result<TokenStream> {
    if cond { then() } else { Ok(quote! {}) }
}

pub(crate) fn drain_attr(attrs: &mut Vec<Attribute>) -> Result<Option<ItemAttr>> {
    let mut ret = None;
    for (i, attr) in attrs.iter().enumerate() {
        let p = attr.path();
        if p.is_ident("prompt") {
            ret = Some((i, ItemAttr::Prompt(parse_args_or_default(attr)?)));
            break;
        }
        if p.is_ident("resource") {
            ret = Some((i, ItemAttr::Resource(parse_args_or_default(attr)?)));
            break;
        }
        if p.is_ident("tool") {
            ret = Some((i, ItemAttr::Tool(parse_args_or_default(attr)?)));
            break;
        }
    }
    let Some((i, arg)) = ret else {
        return Ok(None);
    };
    attrs.remove(i);
    for attr in &attrs[i..] {
        let p = attr.path();
        if p.is_ident("prompt") || p.is_ident("resource") || p.is_ident("tool") {
            bail!(
                attr.span(),
                "Multiple `#[prompt]` or `#[resource]` or `#[tool]` attributes are not allowed"
            );
        }
    }
    Ok(Some(arg))
}

pub(crate) fn parse_args_or_default<T: Parse + Default>(attr: &Attribute) -> Result<T> {
    match &attr.meta {
        syn::Meta::Path(path) => Ok(T::default()),
        syn::Meta::List(_) | syn::Meta::NameValue(_) => attr.parse_args(),
    }
}

pub(crate) fn get_trait_path(item: &ItemImpl) -> Result<&Path> {
    if let Some((not, t, _)) = &item.trait_ {
        if not.is_none() && is_path(t, &[&["mcp_attr", "server"]], "McpServer") {
            return Ok(t);
        }
    }
    bail!(
        item.span(),
        "`#[mcp_server]` can only be used on `impl McpServer for T {{ ... }}`"
    );
}

pub(crate) fn is_defined(items: &[ImplItem], name: &str) -> bool {
    items.iter().any(|item| {
        if let ImplItem::Fn(f) = item {
            f.sig.ident == name
        } else {
            false
        }
    })
}

pub(crate) fn get_only_attr<T: Parse>(attrs: &[Attribute], name: &str) -> Result<Option<T>> {
    for (i, attr) in attrs.iter().enumerate() {
        if attr.path().is_ident(name) {
            for attr in &attrs[i + 1..] {
                if attr.path().is_ident(name) {
                    bail!(
                        attr.span(),
                        "Multiple `#[{name}]` attributes are not allowed"
                    );
                }
            }
            return Ok(Some(attr.parse_args::<T>()?));
        }
    }
    Ok(None)
}
pub(crate) fn take_only_attr<T: Parse>(
    attrs: &mut Vec<Attribute>,
    name: &str,
) -> Result<Option<T>> {
    let mut index = None;
    for (i, attr) in attrs.iter().enumerate() {
        if attr.path().is_ident(name) {
            if index.is_some() {
                bail!(
                    attr.span(),
                    "Multiple `#[{name}]` attributes are not allowed"
                );
            }
            index = Some(i);
        }
    }
    if let Some(index) = index {
        let attr = attrs.remove(index);
        Ok(Some(attr.parse_args::<T>()?))
    } else {
        Ok(None)
    }
}

pub(crate) fn get_doc(attrs: &[Attribute]) -> String {
    let mut lines = Vec::new();
    for attr in attrs {
        if let Some(line) = try_get_doc_from(attr) {
            lines.push(line);
        }
    }
    fix_doc(&lines)
}
pub(crate) fn take_doc(attrs: &mut Vec<Attribute>) -> String {
    let mut lines = Vec::new();
    attrs.retain(|attr| {
        if let Some(line) = try_get_doc_from(attr) {
            lines.push(line);
            false
        } else {
            true
        }
    });
    fix_doc(&lines)
}
fn try_get_doc_from(attr: &Attribute) -> Option<String> {
    if attr.path().is_ident("doc") {
        if let Meta::NameValue(nv) = &attr.meta {
            if let Expr::Lit(lit) = &nv.value {
                if let Lit::Str(lit) = &lit.lit {
                    return Some(lit.value().trim_end().to_string());
                }
            }
        }
    }
    None
}

fn fix_doc(lines: &[String]) -> String {
    let min_indent = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.find(|c| c != ' ').unwrap_or(0))
        .min()
        .unwrap_or(0);
    let mut doc = String::new();
    for line in lines {
        if !doc.is_empty() {
            doc.push('\n');
        }
        if !line.is_empty() {
            doc.push_str(&line[min_indent..]);
        }
    }
    doc
}
#[test]
fn get_doc_test() {
    let item: ItemFn = syn::parse_quote! {
        /// abc
        /// def
        fn func() { }
    };
    let doc = get_doc(&item.attrs);
    assert_eq!(doc, "abc\ndef");
}

pub(crate) fn is_option(ty: &Type) -> bool {
    is_type(ty, &[&["core", "option"], &["std", "option"]], "Option")
}

pub(crate) fn get_option_element(ty: &Type) -> Option<&Type> {
    get_element(ty, &[&["core", "option"], &["std", "option"]], "Option")
}

pub(crate) fn is_context(ty: &Type) -> bool {
    if let Type::Reference(ty) = ty {
        is_type(&ty.elem, &[&["mcp_attr", "server"]], "RequestContext")
    } else {
        false
    }
}

pub(crate) fn expand_option_ty(ty: &Type) -> (Type, bool) {
    if let Some(ty) = get_option_element(ty) {
        (ty.clone(), false)
    } else {
        (ty.clone(), true)
    }
}

pub(crate) fn descriotion_expr(s: &str) -> TokenStream {
    if s.is_empty() {
        quote!(None)
    } else {
        quote!(Some(#s.into()))
    }
}

pub(crate) fn opt_expr<T>(s: &Option<T>, f: impl FnOnce(&T) -> TokenStream) -> TokenStream {
    if let Some(s) = s {
        let s = f(s);
        quote!(Some(#s))
    } else {
        quote!(None)
    }
}

pub(crate) fn arg_name_of(typed_arg: &PatType) -> Result<String> {
    match &*typed_arg.pat {
        Pat::Ident(pat_ident) => {
            let name = pat_ident.ident.to_string();
            if let Some(name) = name.strip_prefix("_") {
                Ok(name.to_string())
            } else {
                Ok(name)
            }
        }
        _ => bail!(typed_arg.pat.span(), "only identifier pattern is supported"),
    }
}

pub(crate) fn ret_span(f: &ImplItemFn) -> Span {
    match &f.sig.output {
        syn::ReturnType::Default => f.span(),
        syn::ReturnType::Type(_, ty) => ty.span(),
    }
}
