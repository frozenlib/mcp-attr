#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use structmeta::{NameArgs, NameValue, StructMeta};
use syn::{
    Attribute, FnArg, Ident, ImplItem, ImplItemFn, ItemFn, ItemImpl, LitStr, Pat, PatType, Path,
    Result, Type, parse::Parse, parse2, spanned::Spanned,
};
use uri_template_ex::UriTemplate;

use crate::utils::{
    arg_name_of, descriotion_expr, expand_option_ty, get_doc, get_only_attr, is_context, ret_span,
    take_doc,
};
use crate::{
    syn_utils::{get_element, is_path, is_type},
    utils::take_only_attr,
};

#[derive(StructMeta, Default)]
pub(crate) struct PromptArgAttr {
    #[struct_meta(unnamed)]
    name: Option<LitStr>,
}

#[derive(StructMeta, Default)]
pub(crate) struct PromptAttr {
    #[struct_meta(unnamed)]
    name: Option<LitStr>,
}

pub(crate) struct PromptEntry {
    name: String,
    fn_ident: Ident,
    description: String,
    args: Vec<PromptFnArg>,
    ret_span: Span,
}
impl PromptEntry {
    pub(crate) fn new(f: &mut ImplItemFn, attr: PromptAttr) -> Result<Self> {
        let name = attr
            .name
            .map(|n| n.value())
            .unwrap_or_else(|| f.sig.ident.to_string());
        let description = get_doc(&f.attrs);
        let args = f
            .sig
            .inputs
            .iter_mut()
            .map(PromptFnArg::new)
            .collect::<Result<Vec<_>>>()?;
        let fn_ident = f.sig.ident.clone();

        Ok(Self {
            name,
            fn_ident,
            description,
            args,
            ret_span: ret_span(f),
        })
    }
    pub fn build_list(items: &[Self]) -> Result<TokenStream> {
        let prompts = items
            .iter()
            .map(|p| p.build_list_items())
            .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            async fn prompts_list(&self,
                p: ::mcp_attr::schema::ListPromptsRequestParams,
                cx: &mut ::mcp_attr::server::RequestContext)
                -> ::mcp_attr::Result<::mcp_attr::schema::ListPromptsResult> {
                    Ok(vec![#(#prompts,)*].into())
            }
        })
    }
    fn build_list_items(&self) -> Result<TokenStream> {
        let name = &self.name;
        let description = descriotion_expr(&self.description);
        let args = self
            .args
            .iter()
            .filter_map(|a| a.build_list_expr().transpose())
            .collect::<Result<Vec<TokenStream>>>()?;
        Ok(quote! {
            ::mcp_attr::schema::Prompt {
                arguments: vec![#(#args,)*],
                name: #name.into(),
                description: #description,
            }
        })
    }
    pub fn build_get(items: &[Self]) -> Result<TokenStream> {
        let arms = items
            .iter()
            .map(|p| p.build_get_arms())
            .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            async fn prompts_get(&self,
                p: ::mcp_attr::schema::GetPromptRequestParams,
                cx: &mut ::mcp_attr::server::RequestContext)
                -> ::mcp_attr::Result<::mcp_attr::schema::GetPromptResult> {
                    match p.name.as_str() {
                        #(#arms)*
                        _ => return ::std::result::Result::Err(::mcp_attr::error::prompt_not_found(&p.name)),
                    }
                }
        })
    }
    fn build_get_arms(&self) -> Result<TokenStream> {
        let name = &self.name;
        let args = self
            .args
            .iter()
            .map(|a| a.build_get_expr())
            .collect::<Result<Vec<_>>>()?;
        let fn_ident = &self.fn_ident;
        let ret_span = self.ret_span;
        Ok(quote_spanned! {ret_span=>
            #name => {
                #[allow(clippy::useless_conversion)]
                {
                    return Ok(<::mcp_attr::schema::GetPromptResult as ::std::convert::From<_>>::from(Self::#fn_ident(#(#args,)*).await?));
                }
            }
        })
    }
}

#[derive(Debug)]
enum PromptFnArg {
    Property(PromptArg),
    Context(Span),
    Receiver(Span),
}
impl PromptFnArg {
    fn new(f: &mut FnArg) -> Result<Self> {
        let span = f.span();
        let typed_arg = match f {
            FnArg::Typed(pat_type) => pat_type,
            FnArg::Receiver(receiver) => return Ok(Self::Receiver(span)),
            _ => bail!(f.span(), "Unsupported function argument pattern"),
        };
        let arg_attr = take_only_attr::<PromptArgAttr>(&mut typed_arg.attrs, "arg")?;
        let has_arg_attr = arg_attr.is_some();
        let arg_attr = arg_attr.unwrap_or_default();
        let description = take_doc(&mut typed_arg.attrs);
        if is_context(&typed_arg.ty) && !has_arg_attr {
            return Ok(Self::Context(span));
        }
        let name = if let Some(name) = &arg_attr.name {
            name.value()
        } else {
            arg_name_of(typed_arg)?
        };
        let (ty, required) = expand_option_ty(&typed_arg.ty);
        Ok(Self::Property(PromptArg {
            name,
            ty,
            description,
            required,
            span,
        }))
    }
    fn build_list_expr(&self) -> Result<Option<TokenStream>> {
        match self {
            Self::Property(arg) => Ok(Some(arg.build_list()?)),
            Self::Context(..) | Self::Receiver(..) => Ok(None),
        }
    }
    fn build_get_expr(&self) -> Result<TokenStream> {
        match self {
            Self::Property(arg) => arg.build_get(),
            Self::Context(span) => Ok(quote_spanned!(*span=> cx)),
            Self::Receiver(span) => Ok(quote_spanned!(*span=> self)),
        }
    }
}

#[derive(Debug)]
struct PromptArg {
    name: String,
    ty: Type,
    description: String,
    required: bool,
    span: Span,
}
impl PromptArg {
    fn build_list(&self) -> Result<TokenStream> {
        let name = &self.name;
        let description = descriotion_expr(&self.description);
        let required = self.required;
        Ok(quote! {
            ::mcp_attr::schema::PromptArgument {
                name: #name.into(),
                description: #description.into(),
                required: Some(#required),
            }
        })
    }
    fn build_get(&self) -> Result<TokenStream> {
        let name = &self.name;
        let ty = &self.ty;
        let span = self.span;
        if self.required {
            Ok(
                quote_spanned!(span=> ::mcp_attr::helpers::parse_prompt_arg::<#ty>(&p.arguments, #name)?),
            )
        } else {
            Ok(
                quote_spanned!(span=> ::mcp_attr::helpers::parse_prompt_arg_opt::<#ty>(&p.arguments, #name)?),
            )
        }
    }
}
