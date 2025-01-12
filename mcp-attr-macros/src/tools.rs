#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use structmeta::{NameArgs, NameValue, StructMeta};
use syn::{
    Attribute, FnArg, Ident, ImplItem, ImplItemFn, ItemFn, ItemImpl, LitStr, Pat, Path, Result,
    Type, parse::Parse, parse2, spanned::Spanned,
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
pub(crate) struct ToolArgAttr {
    #[struct_meta(unnamed)]
    name: Option<LitStr>,
}

#[derive(StructMeta, Default)]
pub(crate) struct ToolAttr {
    #[struct_meta(unnamed)]
    name: Option<LitStr>,
}

pub(crate) struct ToolEntry {
    name: String,
    fn_ident: Ident,
    description: String,
    args: Vec<ToolFnArg>,
    ret_span: Span,
}
impl ToolEntry {
    pub(crate) fn new(f: &mut ImplItemFn, attr: ToolAttr) -> Result<Self> {
        let name = attr
            .name
            .map(|n| n.value())
            .unwrap_or_else(|| f.sig.ident.to_string());
        let fn_ident = f.sig.ident.clone();
        let description = get_doc(&f.attrs);
        let args = f
            .sig
            .inputs
            .iter_mut()
            .map(ToolFnArg::new)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            name,
            fn_ident,
            description,
            args,
            ret_span: ret_span(f),
        })
    }
    pub fn build_list(items: &[Self]) -> Result<TokenStream> {
        let items = items
            .iter()
            .map(|t| t.build_list_items())
            .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            async fn tools_list(&self,
                p: ::mcp_attr::schema::ListToolsRequestParams,
                cx: &mut ::mcp_attr::server::RequestContext)
                -> ::mcp_attr::Result<::mcp_attr::schema::ListToolsResult> {
                    Ok(vec![#(#items,)*].into())
            }
        })
    }
    fn build_list_items(&self) -> Result<TokenStream> {
        let name = &self.name;
        let description = descriotion_expr(&self.description);
        let args = self
            .args
            .iter()
            .map(|a| a.build_list())
            .collect::<Result<Vec<TokenStream>>>()?;
        Ok(quote! {
            {
                let mut input_schema = ::mcp_attr::schema::ToolInputSchema::new();
                #(#args)*
                ::mcp_attr::schema::Tool {
                    name: #name.into(),
                    input_schema,
                    description: #description,
                }
            }
        })
    }
    pub fn build_call(items: &[Self]) -> Result<TokenStream> {
        let arms = items
            .iter()
            .map(|t| t.build_call_arms())
            .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            async fn tools_call(&self,
                p: ::mcp_attr::schema::CallToolRequestParams,
                cx: &mut ::mcp_attr::server::RequestContext)
                -> ::mcp_attr::Result<::mcp_attr::schema::CallToolResult> {
                    match p.name.as_str() {
                        #(#arms)*
                        _ => return ::std::result::Result::Err(::mcp_attr::error::tool_not_found(&p.name)),
                    }
            }
        })
    }
    fn build_call_arms(&self) -> Result<TokenStream> {
        let name = &self.name;
        let args = self
            .args
            .iter()
            .map(|a| a.build_call())
            .collect::<Result<Vec<_>>>()?;
        let fn_ident = &self.fn_ident;
        let ret_span = self.ret_span;
        Ok(quote_spanned! {ret_span=>
            #name => {
                #[allow(clippy::useless_conversion)]
                {
                    return Ok(<::mcp_attr::schema::CallToolResult as ::std::convert::From<_>>::from(Self::#fn_ident(#(#args,)*).await?));
                }
            }
        })
    }
}

enum ToolFnArg {
    Property(ToolArg),
    Context(Span),
    Receiver(Span),
}
impl ToolFnArg {
    fn new(f: &mut FnArg) -> Result<Self> {
        let span = f.span();
        let mut typed_arg = match f {
            FnArg::Typed(pat_type) => pat_type,
            FnArg::Receiver(receiver) => return Ok(Self::Receiver(span)),
            _ => bail!(f.span(), "Unsupported function argument pattern"),
        };
        let arg_attr = take_only_attr::<ToolArgAttr>(&mut typed_arg.attrs, "arg")?;
        let has_arg_attr = arg_attr.is_some();
        let arg_arg = arg_attr.unwrap_or_default();
        let description = take_doc(&mut typed_arg.attrs);
        if is_context(&typed_arg.ty) && !has_arg_attr {
            return Ok(Self::Context(span));
        }
        let name = if let Some(name) = &arg_arg.name {
            name.value()
        } else {
            arg_name_of(typed_arg)?
        };
        let (ty, required) = expand_option_ty(&typed_arg.ty);

        Ok(Self::Property(ToolArg {
            name,
            ty,
            description,
            required,
            span,
        }))
    }
    fn build_list(&self) -> Result<TokenStream> {
        match self {
            Self::Property(arg) => arg.build_list(),
            Self::Context(..) | Self::Receiver(..) => Ok(quote!()),
        }
    }
    fn build_call(&self) -> Result<TokenStream> {
        match self {
            Self::Property(arg) => arg.build_call(),
            Self::Context(span) => Ok(quote_spanned!(*span=> cx)),
            Self::Receiver(span) => Ok(quote_spanned!(*span=> self)),
        }
    }
}

struct ToolArg {
    name: String,
    ty: Type,
    description: String,
    required: bool,
    span: Span,
}
impl ToolArg {
    fn build_list(&self) -> Result<TokenStream> {
        let name = &self.name;
        let ty = &self.ty;
        let description = &self.description;
        let required = self.required;
        Ok(quote! {
            input_schema.insert_property::<#ty>(#name, #description, #required)?;
        })
    }
    fn build_call(&self) -> Result<TokenStream> {
        let name = &self.name;
        let ty = &self.ty;
        let required = self.required;
        let span = self.span;
        if self.required {
            Ok(
                quote_spanned! {span=> ::mcp_attr::helpers::parse_tool_arg::<#ty>(&p.arguments, #name)?},
            )
        } else {
            Ok(
                quote_spanned! {span=> ::mcp_attr::helpers::parse_tool_arg_opt::<#ty>(&p.arguments, #name)?},
            )
        }
    }
}
