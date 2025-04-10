#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use structmeta::{NameArgs, NameValue, StructMeta};
use syn::{
    Attribute, Error, FnArg, Ident, ImplItem, ImplItemFn, ItemFn, ItemImpl, LitStr, Pat, Path,
    Result, Token, Type,
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    spanned::Spanned,
};
use uri_template_ex::UriTemplate;

use syn_utils::{get_element, into_macro_output, is_path, is_type};
use utils::{get_trait_path, is_defined};

use crate::prompts::{PromptAttr, PromptEntry};
use crate::resources::{ResourceAttr, ResourceEntry};
use crate::tools::{ToolAttr, ToolEntry};
use crate::utils::{build_if, drain_attr};

#[macro_use]
mod syn_utils;
mod utils;

mod prompts;
mod resources;
mod tools;

#[proc_macro_attribute]
pub fn mcp_server(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item: TokenStream = item.into();
    let mut es = Vec::new();
    match build_mcp_server(attr.into(), item.clone(), &mut es) {
        Ok(mut s) => {
            for e in es {
                s.extend(e.to_compile_error());
            }
            s
        }
        Err(e) => e.to_compile_error(),
    }
    .into()
}

#[proc_macro]
pub fn route(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    into_macro_output(build_route(item.into()))
}

#[proc_macro_attribute]
pub fn tool(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    into_macro_output(build_tool(attr.into(), item.into()))
}

#[proc_macro_attribute]
pub fn resource(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    into_macro_output(build_resource(attr.into(), item.into()))
}

#[proc_macro_attribute]
pub fn prompt(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    into_macro_output(build_prompt(attr.into(), item.into()))
}

fn build_mcp_server(
    attr: TokenStream,
    item: TokenStream,
    es: &mut Vec<Error>,
) -> Result<TokenStream> {
    let mut item_impl: ItemImpl = parse2(item)?;
    let mut attr: McpAttr = parse2(attr)?;
    let trait_path = get_trait_path(&item_impl)?.clone();
    if item_impl.unsafety.is_some() {
        bail!(item_impl.span(), "Unsafe is not allowed");
    }
    if item_impl.defaultness.is_some() {
        bail!(item_impl.span(), "Default is not allowed");
    }
    let is_defined_resources_list = is_defined(&item_impl.items, "resources_list");
    let mut b = McpBuilder::new();
    let mut items_trait = Vec::new();
    let mut items_type = Vec::new();
    for mut item in item_impl.items {
        match b.push(&mut item) {
            Ok(true) => items_type.push(item),
            Ok(false) => items_trait.push(item),
            Err(e) => {
                items_type.push(item);
                es.push(e);
            }
        }
    }
    let b = b.build(&items_trait)?;
    let (impl_generics, ty_generics, where_clause) = item_impl.generics.split_for_impl();

    let self_ty = &item_impl.self_ty;
    let attrs = &item_impl.attrs;
    let ts = quote! {
        #[automatically_derived]
        #(#attrs)*
        impl<#impl_generics> #trait_path for #self_ty #ty_generics #where_clause {
            #(#items_trait)*
            #b
        }

        #[automatically_derived]
        #(#attrs)*
        impl<#impl_generics> #self_ty #ty_generics #where_clause {
            #(#items_type)*
        }
    };
    if attr.dump {
        dump_code(ts);
    }
    Ok(ts)
}

struct McpBuilder {
    prompts: Vec<PromptEntry>,
    resources: Vec<ResourceEntry>,
    tools: Vec<ToolEntry>,
}

impl McpBuilder {
    fn new() -> Self {
        Self {
            prompts: Vec::new(),
            resources: Vec::new(),
            tools: Vec::new(),
        }
    }
    fn push(&mut self, item: &mut ImplItem) -> Result<bool> {
        if let ImplItem::Fn(f) = item {
            let Some(attr) = drain_attr(&mut f.attrs)? else {
                return Ok(false);
            };
            match attr {
                ItemAttr::Prompt(attr) => {
                    self.prompts.push(PromptEntry::from_impl_item_fn(f, attr)?)
                }
                ItemAttr::Resource(attr) => self
                    .resources
                    .push(ResourceEntry::from_impl_item_fn(f, attr)?),
                ItemAttr::Tool(attr) => self.tools.push(ToolEntry::from_impl_item_fn(f, attr)?),
            }
            return Ok(true);
        }
        Ok(false)
    }

    fn build(&self, items: &[ImplItem]) -> Result<TokenStream> {
        let capabilities = build_if(!is_defined(items, "capabilities"), || {
            self.build_capabilities(items)
        })?;
        let prompts = build_if(!self.prompts.is_empty(), || self.build_prompts())?;
        let resources = build_if(!self.resources.is_empty(), || self.build_resources(items))?;
        let tools = build_if(!self.tools.is_empty(), || self.build_tools())?;
        Ok(quote! {
            #capabilities
            #prompts
            #resources
            #tools
        })
    }
    fn build_capabilities(&self, items: &[ImplItem]) -> Result<TokenStream> {
        let prompts = if !self.prompts.is_empty() || is_defined(items, "prompts_list") {
            quote!(Some(::mcp_attr::schema::ServerCapabilitiesPrompts {
                ..::std::default::Default::default()
            }))
        } else {
            quote!(None)
        };
        let resources = if !self.resources.is_empty() || is_defined(items, "resources_read") {
            quote!(Some(::mcp_attr::schema::ServerCapabilitiesResources {
                ..::std::default::Default::default()
            }))
        } else {
            quote!(None)
        };
        let tools = if !self.tools.is_empty() || is_defined(items, "tools_list") {
            quote!(Some(::mcp_attr::schema::ServerCapabilitiesTools {
                ..::std::default::Default::default()
            }))
        } else {
            quote!(None)
        };
        Ok(quote! {
            fn capabilities(&self) -> ::mcp_attr::schema::ServerCapabilities {
                ::mcp_attr::schema::ServerCapabilities {
                    prompts: #prompts,
                    resources: #resources,
                    tools: #tools,
                    ..::std::default::Default::default()
                }
            }
        })
    }
    fn build_prompts(&self) -> Result<TokenStream> {
        let list = self.build_prompts_list()?;
        let get = self.build_prompts_get()?;
        Ok(quote! {
            #list
            #get
        })
    }
    fn build_resources(&self, items: &[ImplItem]) -> Result<TokenStream> {
        let list = build_if(!is_defined(items, "resources_list"), || {
            self.build_resources_list()
        })?;
        let templates_list = self.build_resources_templates_list()?;
        let read = self.build_resources_read()?;
        Ok(quote! {
            #list
            #templates_list
            #read
        })
    }
    fn build_tools(&self) -> Result<TokenStream> {
        let list = self.build_tools_list()?;
        let call = self.build_tools_call()?;
        Ok(quote! {
            #list
            #call
        })
    }
    fn build_prompts_list(&self) -> Result<TokenStream> {
        PromptEntry::build_list(&self.prompts)
    }
    fn build_prompts_get(&self) -> Result<TokenStream> {
        PromptEntry::build_get(&self.prompts)
    }
    fn build_resources_list(&self) -> Result<TokenStream> {
        ResourceEntry::build_list(&self.resources)
    }
    fn build_resources_templates_list(&self) -> Result<TokenStream> {
        ResourceEntry::build_templates_list(&self.resources)
    }
    fn build_resources_read(&self) -> Result<TokenStream> {
        ResourceEntry::build_read(&self.resources)
    }

    fn build_tools_list(&self) -> Result<TokenStream> {
        ToolEntry::build_list(&self.tools)
    }
    fn build_tools_call(&self) -> Result<TokenStream> {
        ToolEntry::build_call(&self.tools)
    }
}

#[derive(StructMeta, Default)]
struct McpAttr {
    dump: bool,
}

enum ItemAttr {
    Prompt(PromptAttr),
    Resource(ResourceAttr),
    Tool(ToolAttr),
}

fn build_route(item: TokenStream) -> Result<TokenStream> {
    struct PathList(Punctuated<Path, Token![,]>);
    impl Parse for PathList {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(Self(Punctuated::parse_terminated(input)?))
        }
    }
    let mut path_list: PathList = parse2(item)?;
    let mut exprs = Vec::new();
    for mut path in path_list.0 {
        let last = path.segments.last_mut().unwrap();
        let fn_ident = last.ident.clone();
        last.ident = route_ident(&fn_ident);
        last.ident.set_span(Span::call_site());
        exprs.push(quote! {
            {
                let _dummy = #fn_ident; // Ensure rust-analyzer can rename the function.
                ::std::convert::Into::<::mcp_attr::server::builder::Route>::into(#path()?)
            }
        });
    }
    Ok(quote! {
        [#(#exprs),*]
    })
}
fn route_ident(ident: &Ident) -> Ident {
    format_ident!("__route_of_{}", ident)
}
fn build_tool(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let mut f: ItemFn = parse2(item)?;
    let attr: ToolAttr = parse2(attr)?;
    let dump = attr.dump;
    let e = ToolEntry::from_item_fn(&mut f, attr)?;
    let ret = e.build_route();
    Ok(make_extend(f, ret, dump))
}

fn build_resource(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let mut f: ItemFn = parse2(item)?;
    let attr: ResourceAttr = parse2(attr)?;
    let dump = attr.dump;
    let e = ResourceEntry::from_item_fn(&mut f, attr)?;
    let ret = e.build_route();
    Ok(make_extend(f, ret, dump))
}

fn build_prompt(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let mut f: ItemFn = parse2(item)?;
    let attr: PromptAttr = parse2(attr)?;
    let dump = attr.dump;
    let e = PromptEntry::from_item_fn(&mut f, attr)?;
    let ret = e.build_route();
    Ok(make_extend(f, ret, dump))
}

fn make_extend(source: impl ToTokens, code: Result<TokenStream>, dump: bool) -> TokenStream {
    let code = match code {
        Ok(code) => {
            if dump {
                dump_code(code);
            }
            code
        }
        Err(e) => e.to_compile_error(),
    };
    quote! {
        #source
        #code
    }
}
fn dump_code(code: TokenStream) -> ! {
    panic!("// ===== start generated code =====\n{code}\n// ===== end generated code =====\n");
}
