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
    descriotion_expr, expand_option_ty, get_doc, get_only_attr, is_context, opt_expr, ret_span,
    take_doc,
};
use crate::{
    syn_utils::{get_element, is_path, is_type},
    utils::arg_name_of,
};

#[derive(StructMeta, Default)]
pub(crate) struct ResourceAttr {
    #[struct_meta(unnamed)]
    uri: Option<LitStr>,
    name: Option<LitStr>,
    mime_type: Option<LitStr>,
}

pub(crate) struct ResourceEntry {
    uri: Option<UriTemplate>,
    name: String,
    mime_type: Option<String>,
    description: String,
    args: Vec<ResourceFnArg>,
    fn_ident: Ident,
    ret_span: Span,
}

impl ResourceEntry {
    pub(crate) fn new(f: &mut ImplItemFn, attr: ResourceAttr) -> Result<Self> {
        let mut name = None;
        let mut uri = None;
        let mut mime_type = None;
        if let Some(attr_uri) = &attr.uri {
            let uri_value = attr_uri.value();
            uri = match UriTemplate::new(&uri_value) {
                Ok(uri) => Some(uri),
                Err(e) => {
                    bail!(attr_uri.span(), "Invalid URI template: `{uri_value}` ({e})",)
                }
            };
            name = attr.name.map(|n| n.value());
            mime_type = attr.mime_type.map(|m| m.value());
        }
        let name = name.unwrap_or_else(|| f.sig.ident.to_string());
        let description = take_doc(&mut f.attrs);
        let args = f
            .sig
            .inputs
            .iter()
            .map(|f| ResourceFnArg::new(f, &uri))
            .collect::<Result<Vec<_>>>()?;
        let fn_ident = f.sig.ident.clone();
        Ok(Self {
            name,
            uri,
            mime_type,
            description,
            args,
            fn_ident,
            ret_span: ret_span(f),
        })
    }
    pub fn build_list(items: &[Self]) -> Result<TokenStream> {
        let arms = items
            .iter()
            .filter_map(|r| r.build_list_arm().transpose())
            .collect::<Result<Vec<TokenStream>>>()?;
        Ok(quote! {
            async fn resources_list(&self,
                p: ::mcp_attr::schema::ListResourcesRequestParams,
                cx: &mut ::mcp_attr::server::RequestContext)
                -> ::mcp_attr::Result<::mcp_attr::schema::ListResourcesResult> {
                    Ok(vec![#(#arms,)*].into())
            }
        })
    }

    fn build_list_arm(&self) -> Result<Option<TokenStream>> {
        let Some(uri) = self.uri.as_ref() else {
            return Ok(None);
        };
        if uri.var_names().count() > 0 {
            return Ok(None);
        }
        let name = &self.name;
        let uri = uri.expand(());
        let mime_type = opt_expr(&self.mime_type, |x| quote!(#x.to_string()));
        let description = descriotion_expr(&self.description);
        Ok(Some(quote! {
            ::mcp_attr::schema::Resource {
                name: #name.to_string(),
                uri: #uri.to_string(),
                mime_type: #mime_type,
                description: #description,
                size: None,
                annotations: None,
            }
        }))
    }

    pub fn build_templates_list(items: &[Self]) -> Result<TokenStream> {
        let arms = items
            .iter()
            .filter_map(|r| r.build_templates_list_arm().transpose())
            .collect::<Result<Vec<TokenStream>>>()?;
        Ok(quote! {
            async fn resources_templates_list(&self,
                p: ::mcp_attr::schema::ListResourceTemplatesRequestParams,
                cx: &mut ::mcp_attr::server::RequestContext)
                -> ::mcp_attr::Result<::mcp_attr::schema::ListResourceTemplatesResult> {
                    Ok(vec![#(#arms,)*].into())
            }
        })
    }

    fn build_templates_list_arm(&self) -> Result<Option<TokenStream>> {
        let Some(uri) = self.uri.as_ref() else {
            return Ok(None);
        };
        if uri.var_names().count() == 0 {
            return Ok(None);
        }
        let name = &self.name;
        let uri = uri.to_string();
        let mime_type = opt_expr(&self.mime_type, |x| quote!(#x.to_string()));
        let description = descriotion_expr(&self.description);
        Ok(Some(quote! {
            ::mcp_attr::schema::ResourceTemplate {
                name: #name.to_string(),
                uri_template: #uri.to_string(),
                mime_type: #mime_type,
                description: #description,
                annotations: None,
            }
        }))
    }

    pub fn build_read(items: &[Self]) -> Result<TokenStream> {
        let stmts = items
            .iter()
            .map(|r| r.build_read_stmt())
            .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            #[allow(unreachable_code)]
            async fn resources_read(&self,
                p: ::mcp_attr::schema::ReadResourceRequestParams,
                cx: &mut ::mcp_attr::server::RequestContext)
                -> ::mcp_attr::Result<::mcp_attr::schema::ReadResourceResult> {
                    #(#stmts)*
                    ::mcp_attr::bail_public!(::mcp_attr::ErrorCode::INVALID_PARAMS, "resource `{}` not found", p.uri);
                }
        })
    }
    fn build_read_stmt(&self) -> Result<Option<TokenStream>> {
        let description = descriotion_expr(&self.description);
        let name = &self.name;
        let mime_type = opt_expr(&self.mime_type, |x| quote!(#x.to_string()));
        let args = self
            .args
            .iter()
            .map(|a| a.build_read())
            .collect::<Result<Vec<TokenStream>>>()?;
        let fn_ident = &self.fn_ident;
        let ret_span = self.ret_span;
        if let Some(uri) = self.uri.as_ref() {
            let uri = uri.to_string();
            Ok(Some(quote_spanned! {ret_span=>
                {
                    static URI_TEMPLATE : ::std::sync::LazyLock<::mcp_attr::helpers::uri_template_ex::UriTemplate> =
                        ::std::sync::LazyLock::new(|| ::mcp_attr::helpers::uri_template_ex::UriTemplate::new(#uri).unwrap());
                    if let Some(_captures) = URI_TEMPLATE.captures(&p.uri) {
                        #[allow(clippy::useless_conversion)]
                        {
                            return Ok(<::mcp_attr::schema::ReadResourceResult as ::std::convert::From<_>>::from(Self::#fn_ident(#(#args,)*).await?));
                        }
                    }
                }
            }))
        } else {
            Ok(Some(quote_spanned! {ret_span=>
                #[allow(clippy::useless_conversion)]
                {
                    return Ok(<::mcp_attr::schema::ReadResourceResult as ::std::convert::From<_>>::from(Self::#fn_ident(#(#args,)*).await?));
                }
            }))
        }
    }
}

enum ResourceFnArg {
    Receiver(Span),
    Context(Span),
    Url(Type, Span),
    Var(UriVar),
}

impl ResourceFnArg {
    fn new(f: &FnArg, uri: &Option<UriTemplate>) -> Result<Self> {
        let span = f.span();
        let typed_arg = match f {
            FnArg::Typed(pat_type) => pat_type,
            FnArg::Receiver(_) => return Ok(Self::Receiver(span)),
        };
        if is_context(&typed_arg.ty) {
            return Ok(Self::Context(span));
        }
        if let Some(uri) = uri {
            let name = arg_name_of(typed_arg)?;
            if let Some(index) = uri.find_var_name(&name) {
                let (ty, required) = expand_option_ty(&typed_arg.ty);
                Ok(Self::Var(UriVar {
                    name,
                    index,
                    ty,
                    required,
                    span,
                }))
            } else {
                bail!(span, "URL Template does not contain variable `{name}`")
            }
        } else {
            Ok(Self::Url((*typed_arg.ty).clone(), span))
        }
    }
    fn build_read(&self) -> Result<TokenStream> {
        match self {
            ResourceFnArg::Receiver(span) => Ok(quote_spanned!(*span=> self)),
            ResourceFnArg::Context(span) => Ok(quote_spanned!(*span=> cx)),
            ResourceFnArg::Url(ty, span) => {
                Ok(quote_spanned!(*span=> ::std::convert::Into::<#ty>::into(&p.uri)))
            }
            ResourceFnArg::Var(x) => x.build_read(),
        }
    }
}

struct UriVar {
    name: String,
    index: usize,
    ty: Type,
    required: bool,
    span: Span,
}
impl UriVar {
    fn build_read(&self) -> Result<TokenStream> {
        let name = &self.name;
        let index = self.index;
        let ty = &self.ty;
        let span = self.span;
        Ok(if self.required {
            quote_spanned!(span=> ::mcp_attr::helpers::parse_resource_arg::<#ty>(&_captures, #index, #name)?)
        } else {
            quote_spanned!(span=> ::mcp_attr::helpers::parse_resource_arg_opt::<#ty>(&_captures, #index, #name)?)
        })
    }
}
