use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::fs::write;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::slice;
use std::sync::LazyLock;

use annotate_snippets::Level;
use annotate_snippets::Renderer;
use annotate_snippets::Snippet;
use anyhow::bail;
use anyhow::Result;
use heck::ToPascalCase;
use heck::ToSnakeCase;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;

fn main() -> Result<()> {
    let code = rustfmt(&build_code()?);
    write("./src/schema.rs", code)?;
    Ok(())
}

fn read_schema() -> Result<Schema> {
    let path = "./schema/schema.json";
    let text = fs::read_to_string(path)?;
    match serde_json::from_str(&text) {
        Ok(schema) => Ok(schema),
        Err(e) => {
            let offset = to_offset(&text, e.line(), e.column());
            let title = e.to_string().lines().next().unwrap_or_default().to_string();
            let m = Level::Error.title(&title).snippet(
                Snippet::source(&text)
                    .fold(true)
                    .origin(path)
                    .annotation(Level::Error.span(offset..offset)),
            );
            let renderer = Renderer::styled();
            eprintln!("{}", renderer.render(m));
            Err(e.into())
        }
    }
}
fn to_offset(text: &str, line: usize, column: usize) -> usize {
    text.lines()
        .take(line - 1)
        .map(|l| l.len() + 1)
        .sum::<usize>()
        + column
}

fn build_code() -> Result<String> {
    let schema = read_schema()?;
    let mut ts = TokenStream::new();
    ts.extend(quote! {
        #![allow(rustdoc::bare_urls)]
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;
        use crate::utils::Integer;
    });
    for (name, d) in &schema.definitions {
        d.build_type(&Ctx::new(name), &mut ts)?;
    }
    let version = schema.schema;
    let head=format!("
//! Model Context Protocol Types
//! 
//! This code is automatically generated from [Model Context Protocol Schema](https://github.com/modelcontextprotocol/specification/blob/main/schema/schema.json).
//! 
//! Schema: <{version}>

");
    Ok(format!("{head}{ts}"))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[allow(unused)]
struct Schema {
    #[serde(rename = "$schema")]
    schema: String,
    definitions: HashMap<String, Definition>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[allow(unused)]
struct Definition {
    description: Option<String>,
    properties: Option<HashMap<String, Definition>>,
    items: Option<Box<Definition>>,
    r#type: Option<Type>,
    #[serde(rename = "$ref")]
    r#ref: Option<String>,
    maximum: Option<f64>,
    minimum: Option<f64>,
    format: Option<Format>,
    any_of: Option<Vec<Definition>>,
    required: Option<Vec<String>>,
    r#const: Option<String>,
    additional_properties: Option<AdditionalProperties>,
    r#enum: Option<Vec<String>>,
}
impl Definition {
    fn build_type(&self, ctx: &Ctx, ts: &mut TokenStream) -> Result<Ident> {
        match &self.r#type {
            None => {
                if let Some(any_of) = &self.any_of {
                    self.build_enum(any_of, ctx, ts)
                } else if let Some(path) = &self.r#ref {
                    ref_path_to_type_name(path)
                } else {
                    bail!("{ctx}: missing any_of or ref");
                }
            }
            Some(Type::Object) => self.build_struct(ctx, ts),
            Some(Type::Enum(tys)) => self.build_enum_tys(tys, ctx, ts),
            Some(ty) => self.build_new_type(ty, ctx, ts),
        }
    }
    fn build_enum(&self, any_of: &[Definition], ctx: &Ctx, ts: &mut TokenStream) -> Result<Ident> {
        let variants: Result<Vec<_>> = any_of.iter().map(|d| d.to_ref_type(ctx)).collect();
        let variants = variants?;
        let ident = ctx.type_ident();
        let doc = to_doc_comemnt_opt(&self.description);
        ts.extend(quote! {
            #doc
            #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
            #[serde(untagged)]
            pub enum #ident {
                #(#variants(#variants),)*
            }
        });
        Ok(ident)
    }
    fn build_struct(&self, ctx: &Ctx, ts: &mut TokenStream) -> Result<Ident> {
        let Some(properties) = &self.properties else {
            bail!("{ctx}: missing properties");
        };
        let mut required = HashSet::new();
        if let Some(req) = &self.required {
            for r in req {
                required.insert(r);
            }
        }
        let fields: Result<Vec<_>> = properties
            .iter()
            .map(|(name, d)| d.to_field(name, required.contains(name), ctx, ts))
            .collect();
        let fields = fields?;
        let ident = ctx.type_ident();
        let const_defs = fields.iter().filter_map(|f| f.const_def());
        let fn_new_args: Vec<_> = fields.iter().filter_map(|f| f.fn_new_arg()).collect();
        let ctor_args = fields.iter().map(|f| f.ctor_arg());
        let derive_default = if fn_new_args.is_empty() {
            quote!(#[derive(Default)])
        } else {
            quote!()
        };
        let doc = to_doc_comemnt_opt(&self.description);
        let mut additional_properties = quote!();
        let mut additional_properties_ctor_arg = quote!();
        if let Some(a) = &self.additional_properties {
            if a.is_allow() {
                additional_properties = quote! {
                    #[serde(flatten)]
                    pub additional_properties: HashMap<String, serde_json::Value>,
                };
                additional_properties_ctor_arg = quote! {
                    additional_properties: HashMap::new(),
                };
            }
        }

        ts.extend(quote! {
            #doc
            #derive_default
            #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
            pub struct #ident {
                #(#fields)*
                #additional_properties
            }
            impl #ident {
                #(#const_defs)*
                pub fn new(#(#fn_new_args),*) -> Self {
                    Self {
                        #(#ctor_args,)*
                        #additional_properties_ctor_arg
                    }
                }
            }
        });
        Ok(ident)
    }
    fn build_enum_tys(&self, tys: &[Type], ctx: &Ctx, ts: &mut TokenStream) -> Result<Ident> {
        let vs: Result<Vec<_>> = tys.iter().map(|ty| ty.to_variant(ctx)).collect();
        let vs = vs?;
        let ident = ctx.type_ident();
        let doc = to_doc_comemnt_opt(&self.description);
        ts.extend(quote! {
            #doc
            #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
            pub enum #ident {
                #(#vs,)*
            }
        });
        Ok(ident)
    }
    fn build_new_type(&self, ty: &Type, ctx: &Ctx, ts: &mut TokenStream) -> Result<Ident> {
        let tys = slice::from_ref(ty);
        self.build_enum_tys(tys, ctx, ts)
    }
    fn to_ref_type(&self, ctx: &Ctx) -> Result<Ident> {
        if let Some(r) = &self.r#ref {
            ref_path_to_type_name(r)
        } else {
            bail!("{ctx}: missing ref");
        }
    }
    fn to_field(
        &self,
        name: &str,
        required: bool,
        ctx: &Ctx,
        ts: &mut TokenStream,
    ) -> Result<FieldEntry> {
        let ctx = &ctx.with(&name.to_pascal_case());
        let mut ty = self.to_type(ctx, ts)?;
        if !required {
            ty = quote!(Option<#ty>);
        }
        Ok(FieldEntry {
            doc: self.description.clone(),
            name: name.to_string(),
            ty,
            required,
            const_value: self.r#const.clone(),
        })
    }
    fn to_type(&self, ctx: &Ctx, ts: &mut TokenStream) -> Result<TokenStream> {
        if let Some(ty) = &self.r#type {
            self.to_type_with(ty, ctx, ts)
        } else if self.r#ref.is_some() {
            let name = self.to_ref_type(ctx)?;
            Ok(quote!(#name))
        } else if let Some(any_of) = &self.any_of {
            let ident = self.build_enum(any_of, ctx, ts)?;
            Ok(quote!(#ident))
        } else {
            Ok(quote!(serde_json::Value))
        }
    }
    fn to_type_with(&self, ty: &Type, ctx: &Ctx, ts: &mut TokenStream) -> Result<TokenStream> {
        Ok(match ty {
            Type::Object => {
                if self.properties.is_some() {
                    let ident = self.build_struct(ctx, ts)?;
                    quote!(#ident)
                } else {
                    quote!(serde_json::Value)
                }
            }
            Type::Array => {
                let Some(items) = &self.items else {
                    bail!("{ctx}: missing item element type.");
                };
                let ty = items.to_type(ctx, ts)?;
                quote!(Vec<#ty>)
            }
            Type::String => quote!(String),
            Type::Boolean => quote!(bool),
            Type::Number => quote!(f64),
            Type::Integer => quote!(Integer),
            Type::Enum(tys) => {
                let tys: Result<Vec<_>> = tys
                    .iter()
                    .map(|ty| self.to_type_with(ty, ctx, ts))
                    .collect();
                let tys = tys?;
                quote!((#(#tys,)*))
            }
        })
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
enum Type {
    Object,
    String,
    Array,
    Boolean,
    Number,
    Integer,
    #[serde(untagged)]
    Enum(Vec<Type>),
}
impl Type {
    fn to_variant(&self, ctx: &Ctx) -> Result<TokenStream> {
        let ctx = ctx.with(&format!("{self:?}"));

        Ok(match self {
            Type::String => quote!(String(String)),
            Type::Boolean => quote!(Bool(bool)),
            Type::Number => quote!(F64(f64)),
            Type::Integer => quote!(Integer(Integer)),
            Type::Object | Type::Array | Type::Enum(_) => bail!("{ctx}: Unsupported type."),
        })
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Format {
    Byte,
    Uri,
    #[serde(rename = "uri-template")]
    UriTemplate,
}

fn ref_path_to_type_name(s: &str) -> Result<Ident> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^#/definitions/(.*)$").unwrap());
    match RE.captures(s) {
        Some(c) => {
            let name = c.get(1).unwrap().as_str().to_string();
            Ok(format_ident!("{name}"))
        }
        None => bail!("invalid ref: {s}"),
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum AdditionalProperties {
    Bool(bool),
    Object(#[allow(unused)] HashMap<String, Value>),
}

impl AdditionalProperties {
    fn is_allow(&self) -> bool {
        match self {
            AdditionalProperties::Bool(allow) => *allow,
            AdditionalProperties::Object(_) => true,
        }
    }
}

struct FieldEntry {
    doc: Option<String>,
    name: String,
    ty: TokenStream,
    required: bool,
    const_value: Option<String>,
}
impl ToTokens for FieldEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let doc = to_doc_comemnt_opt(&self.doc);
        let name = &self.name;
        let ident = self.name_ident();
        let ty = &self.ty;
        tokens.extend(quote! {
            #doc
            #[serde(rename = #name)]
            pub #ident: #ty,
        })
    }
}
impl FieldEntry {
    fn name_ident(&self) -> Ident {
        format_ident!("r#{}", self.name.to_snake_case())
    }
    fn fn_new_arg(&self) -> Option<TokenStream> {
        if self.required && self.const_value.is_none() {
            let ident = self.name_ident();
            let ty = &self.ty;
            Some(quote!(#ident: #ty))
        } else {
            None
        }
    }
    fn ctor_arg(&self) -> TokenStream {
        let name = self.name_ident();
        if self.required {
            if let Some(c) = &self.const_value {
                quote!(#name: #c.to_string())
            } else {
                quote!(#name)
            }
        } else {
            quote!(#name: None)
        }
    }

    fn const_def(&self) -> Option<TokenStream> {
        let c = self.const_value.as_ref()?;
        let ident = format_ident!("{}", self.name.to_uppercase());
        Some(quote!(pub const #ident: &str = #c;))
    }
}

#[derive(Debug)]
struct Ctx(Vec<String>);

impl Ctx {
    fn new(name: &str) -> Self {
        Self(vec![name.to_string()])
    }
    fn with(&self, name: &str) -> Ctx {
        let mut names = self.0.clone();
        names.push(name.to_string());
        Ctx(names)
    }
    fn type_ident(&self) -> Ident {
        format_ident!("{}", self.0.join(""))
    }
}
impl fmt::Display for Ctx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join("/"))
    }
}

fn to_doc_comemnt_opt(s: &Option<String>) -> TokenStream {
    match s {
        Some(s) => to_doc_comemnt(s),
        None => TokenStream::new(),
    }
}
fn to_doc_comemnt(s: &str) -> TokenStream {
    let s = s.trim();
    let mut ts = TokenStream::new();
    for line in s.lines() {
        let line = format!(" {line}");
        ts.extend(quote!(#[doc = #line]));
    }
    ts
}

fn rustfmt(s: &str) -> String {
    match rustfmt_raw(s) {
        Ok(s) => s,
        Err(_) => s.replace("}", "}\n"),
    }
}
fn rustfmt_raw(s: &str) -> Result<String> {
    let child = Command::new("rustfmt")
        .args(["--config", "normalize_doc_attributes=true"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child.stdin.as_ref().unwrap().write_all(s.as_bytes())?;
    let o = child.wait_with_output()?;
    if o.status.success() {
        Ok(std::str::from_utf8(&o.stdout)?.to_string())
    } else {
        bail!("{}", std::str::from_utf8(&o.stderr)?);
    }
}
