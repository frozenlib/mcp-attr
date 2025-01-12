use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, GenericArgument, Path, PathArguments, PathSegment, Result, Token, Type,
    WherePredicate, punctuated::Punctuated,
};

macro_rules! bail {
    (_, $($arg:tt)*) => {
        bail!(proc_macro2::Span::call_site(), $($arg)*)
    };
    ($span:expr, $fmt:expr $(,)?) => {
        return std::result::Result::Err(syn::Error::new($span, std::format!($fmt)))
    };
    ($span:expr, $fmt:expr, $($arg:tt)*) => {
        return std::result::Result::Err(syn::Error::new($span, std::format!($fmt, $($arg)*)))
    };
}

pub fn is_type(ty: &Type, ns: &[&[&str]], name: &str) -> bool {
    if let Some(a) = get_arguments_of_type(ty, ns, name) {
        a.is_empty()
    } else {
        false
    }
}
pub fn get_arguments_of_type<'a>(
    ty: &'a Type,
    ns: &[&[&str]],
    name: &str,
) -> Option<&'a PathArguments> {
    if let Type::Path(ty) = ty {
        if ty.qself.is_none() {
            return get_arguments_of_path(&ty.path, ns, name);
        }
    }
    None
}

pub fn is_path(path: &Path, ns: &[&[&str]], name: &str) -> bool {
    if let Some(a) = get_arguments_of_path(path, ns, name) {
        a.is_empty()
    } else {
        false
    }
}
pub fn get_arguments_of_path<'a>(
    path: &'a Path,
    ns: &[&[&str]],
    name: &str,
) -> Option<&'a PathArguments> {
    let ss = &path.segments;
    if let Some(last) = ss.last() {
        if last.ident != name {
            return None;
        }
        return if ns.iter().any(|ns| is_match_ns(ss, ns)) {
            Some(&last.arguments)
        } else {
            None
        };
    }

    None
}
pub fn is_match_ns(ss: &Punctuated<PathSegment, Token![::]>, ns: &[&str]) -> bool {
    let mut i_ss = ss.len() - 1;
    let mut i_ns = ns.len();
    while i_ss > 0 && i_ns > 0 {
        i_ns -= 1;
        i_ss -= 1;
        let s = &ss[i_ss];
        if s.ident != ns[i_ns] || !s.arguments.is_empty() {
            return false;
        }
    }
    i_ss == 0
}

pub fn get_element<'a>(ty: &'a Type, ns: &[&[&str]], name: &str) -> Option<&'a Type> {
    if let PathArguments::AngleBracketed(args) = get_arguments_of_type(ty, ns, name)? {
        if args.args.len() == 1 {
            if let GenericArgument::Type(ty) = &args.args[0] {
                return Some(ty);
            }
        }
    }
    None
}
