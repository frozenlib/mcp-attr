use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use schemars::schema::{InstanceType, SchemaObject};
use std::fs;
use typify::{TypeSpace, TypeSpaceSettings};

use crate::utils::{push_derive, ts_replace};

pub fn build_schema(input: &str) -> Result<TokenStream> {
    let s = fs::read_to_string(input)?;
    let mut ts = TypeSpace::new(
        TypeSpaceSettings::default()
            .with_replacement("RequestId", "RequestId", [].into_iter())
            .with_replacement("ProgressToken", "RequestId", [].into_iter())
            .with_conversion(
                SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    format: Some("byte".to_string()),
                    ..Default::default()
                },
                "crate::utils::Base64Bytes",
                [].into_iter(),
            )
            .with_derive("PartialEq".into())
            .with_map_type("::std::collections::BTreeMap"),
    );
    ts.add_root_schema(serde_json::from_str(&s)?)?;

    // let file: File = parse2(quote! {#ts})?;
    // let derive_default_types = derive_default_types(&file)?;
    let mut ts = quote! {
        #![allow(rustdoc::bare_urls)]
        #![allow(clippy::derivable_impls)]
        #![allow(clippy::clone_on_copy)]
        #![allow(irrefutable_let_patterns)]
        pub use jsoncall::RequestId;
        #ts
    };
    // for ty in &derive_default_types {
    //     push_derive(&ty.to_string(), &["Default"], &mut ts)?;
    // }
    for ty in &[
        "ListToolsResult",
        "ListResourceTemplatesResult",
        "CompleteResultCompletion",
        "ListPromptsResult",
        "GetPromptResult",
        "ListResourcesResult",
        "GetResourceResult",
        "GetResourceTemplateResult",
        "GetToolsResult",
        "CompleteResult",
        "TextResourceContents",
        "BlobResourceContents",
    ] {
        push_derive(ty, &["Default"], &mut ts)?;
    }

    ts_replace(
        r#"
#[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
pub $id:ident : ::serde_json::Map<::std::string::String, ::serde_json::Value>,
"#,
        r#"
#[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
pub $id: ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        "#,
        &mut ts,
    )?;

    Ok(ts)
}
