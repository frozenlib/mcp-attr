#![allow(missing_docs)]

use jsoncall::{ErrorCode, bail_public};
use schemars::{JsonSchema, schema::Metadata, schema_for};
use serde::Serialize;
use serde_json::{Value, to_value};
use url::Url;

use crate::{
    Result,
    schema::{
        BlobResourceContents, CallToolRequestParams, CallToolResult, CallToolResultContentItem,
        CompleteRequestParams, CompleteRequestParamsArgument, CompleteRequestParamsRef,
        CompleteResult, CompleteResultCompletion, EmbeddedResource, EmbeddedResourceResource,
        GetPromptRequestParams, GetPromptResult, ImageContent, Implementation, ListPromptsResult,
        ListResourceTemplatesResult, ListResourcesResult, ListRootsResult, ListToolsResult, Prompt,
        PromptArgument, PromptMessage, PromptMessageContent, PromptReference,
        ReadResourceRequestParams, ReadResourceResult, ReadResourceResultContentsItem, Resource,
        ResourceAnnotations, ResourceReference, ResourceTemplate, ResourceTemplateAnnotations,
        Role, Root, TextContent, TextResourceContents, Tool, ToolInputSchema,
    },
    utils::Base64Bytes,
};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
use std::{fmt::Display, str::FromStr};

impl From<Vec<Prompt>> for ListPromptsResult {
    fn from(prompts: Vec<Prompt>) -> Self {
        ListPromptsResult {
            prompts,
            next_cursor: None,
            meta: Default::default(),
        }
    }
}
impl<T: Into<PromptMessage>> From<Vec<T>> for GetPromptResult {
    fn from(messages: Vec<T>) -> Self {
        GetPromptResult {
            description: None,
            messages: messages.into_iter().map(|m| m.into()).collect(),
            meta: Default::default(),
        }
    }
}
impl From<PromptMessage> for GetPromptResult {
    fn from(message: PromptMessage) -> Self {
        vec![message].into()
    }
}
impl From<PromptMessageContent> for PromptMessage {
    fn from(content: PromptMessageContent) -> Self {
        PromptMessage {
            content,
            role: Role::User,
        }
    }
}
impl From<Vec<Resource>> for ListResourcesResult {
    fn from(resources: Vec<Resource>) -> Self {
        ListResourcesResult {
            resources,
            next_cursor: None,
            meta: Default::default(),
        }
    }
}
impl From<Vec<ResourceTemplate>> for ListResourceTemplatesResult {
    fn from(resource_templates: Vec<ResourceTemplate>) -> Self {
        ListResourceTemplatesResult {
            resource_templates,
            next_cursor: None,
            meta: Default::default(),
        }
    }
}
impl From<Vec<ReadResourceResultContentsItem>> for ReadResourceResult {
    fn from(contents: Vec<ReadResourceResultContentsItem>) -> Self {
        ReadResourceResult {
            contents,
            meta: Default::default(),
        }
    }
}
impl From<ReadResourceResultContentsItem> for ReadResourceResult {
    fn from(content: ReadResourceResultContentsItem) -> Self {
        ReadResourceResult {
            contents: vec![content],
            meta: Default::default(),
        }
    }
}

impl From<Vec<Tool>> for ListToolsResult {
    fn from(tools: Vec<Tool>) -> Self {
        ListToolsResult {
            tools,
            next_cursor: None,
            meta: Default::default(),
        }
    }
}
impl<T: Into<CallToolResultContentItem>> From<Vec<T>> for CallToolResult {
    fn from(content: Vec<T>) -> Self {
        CallToolResult {
            content: content.into_iter().map(|c| c.into()).collect(),
            is_error: None,
            meta: Default::default(),
        }
    }
}
impl From<()> for CallToolResult {
    fn from(_: ()) -> Self {
        Vec::<CallToolResultContentItem>::new().into()
    }
}

impl From<CallToolResultContentItem> for CallToolResult {
    fn from(content: CallToolResultContentItem) -> Self {
        vec![content].into()
    }
}
impl GetPromptRequestParams {
    pub fn new(name: &str) -> Self {
        GetPromptRequestParams {
            name: name.to_string(),
            arguments: BTreeMap::new(),
        }
    }
    pub fn with_arguments<K, V>(mut self, arguments: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Display,
        V: Display,
    {
        self.arguments = arguments
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }
}

impl Prompt {
    pub fn new(name: &str) -> Self {
        Prompt {
            name: name.to_string(),
            arguments: vec![],
            description: None,
        }
    }
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
    pub fn with_arguments(mut self, arguments: Vec<PromptArgument>) -> Self {
        self.arguments = arguments;
        self
    }
}
impl PromptArgument {
    pub fn new(name: &str, required: bool) -> Self {
        PromptArgument {
            name: name.to_string(),
            description: None,
            required: Some(required),
        }
    }
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }
}

impl Resource {
    pub fn new(uri: &str, name: &str) -> Self {
        Resource {
            uri: uri.to_string(),
            name: name.to_string(),
            description: None,
            mime_type: None,
            annotations: None,
            size: None,
        }
    }
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
    pub fn with_mime_type(mut self, mime_type: &str) -> Self {
        self.mime_type = Some(mime_type.to_string());
        self
    }
    pub fn with_annotations(mut self, annotations: impl Into<ResourceAnnotations>) -> Self {
        self.annotations = Some(annotations.into());
        self
    }
    pub fn with_size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
}
impl ResourceTemplate {
    pub fn new(uri_template: &str, name: &str) -> Self {
        ResourceTemplate {
            uri_template: uri_template.to_string(),
            name: name.to_string(),
            annotations: None,
            description: None,
            mime_type: None,
        }
    }
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
    pub fn with_mime_type(mut self, mime_type: &str) -> Self {
        self.mime_type = Some(mime_type.to_string());
        self
    }
    pub fn with_annotations(mut self, annotations: impl Into<ResourceTemplateAnnotations>) -> Self {
        self.annotations = Some(annotations.into());
        self
    }
}
impl ReadResourceRequestParams {
    pub fn new(uri: &str) -> Self {
        ReadResourceRequestParams {
            uri: uri.to_string(),
        }
    }
}

impl Tool {
    pub fn new(name: &str, input_schema: ToolInputSchema) -> Self {
        Tool {
            name: name.to_string(),
            description: None,
            input_schema,
        }
    }
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
}

impl ToolInputSchema {
    pub fn new() -> Self {
        Self {
            properties: BTreeMap::new(),
            required: vec![],
            type_: "object".to_string(),
        }
    }
    pub fn insert_property<T: JsonSchema>(
        &mut self,
        name: &str,
        description: &str,
        required: bool,
    ) -> Result<()> {
        let mut root = schema_for!(T);
        if !description.is_empty() {
            let metadata = root
                .schema
                .metadata
                .get_or_insert(Box::new(Metadata::default()));
            metadata.description = Some(description.to_string());
        }
        let value = to_value(root.schema)?;
        let Value::Object(obj) = value else {
            bail_public!(
                ErrorCode::INVALID_PARAMS,
                "schema for `{name}` is not an object"
            );
        };
        self.properties.insert(name.to_string(), obj);
        if required {
            self.required.push(name.to_string());
        }
        Ok(())
    }
    pub fn with_property<T: JsonSchema>(
        mut self,
        name: &str,
        description: &str,
        required: bool,
    ) -> Result<Self> {
        self.insert_property::<T>(name, description, required)?;
        Ok(self)
    }
}
impl Default for ToolInputSchema {
    fn default() -> Self {
        Self::new()
    }
}
impl CallToolRequestParams {
    pub fn new(name: &str) -> Self {
        CallToolRequestParams {
            name: name.to_string(),
            arguments: None,
        }
    }
    pub fn with_argument(mut self, name: &str, value: impl Serialize) -> Result<Self> {
        let mut arguments = self.arguments.unwrap_or_default();
        arguments.insert(name.to_string(), to_value(value)?);
        self.arguments = Some(arguments);
        Ok(self)
    }
}

impl TextContent {
    pub fn new(text: impl std::fmt::Display) -> Self {
        Self {
            text: text.to_string(),
            annotations: None,
            type_: "text".to_string(),
        }
    }
}
impl From<String> for TextContent {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}
impl From<&str> for TextContent {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl ImageContent {
    pub fn new(data: Base64Bytes, mime_type: &str) -> Self {
        Self {
            data,
            mime_type: mime_type.to_string(),
            annotations: None,
            type_: "image".to_string(),
        }
    }
}

impl EmbeddedResource {
    pub fn new(resource: impl Into<EmbeddedResourceResource>) -> Self {
        Self {
            annotations: None,
            resource: resource.into(),
            type_: "resource".to_string(),
        }
    }
}

impl From<String> for TextResourceContents {
    fn from(text: String) -> Self {
        TextResourceContents {
            text,
            ..Default::default()
        }
    }
}
impl From<&str> for TextResourceContents {
    fn from(text: &str) -> Self {
        text.to_string().into()
    }
}

impl From<Base64Bytes> for BlobResourceContents {
    fn from(blob: Base64Bytes) -> Self {
        BlobResourceContents {
            blob,
            ..Default::default()
        }
    }
}

impl Implementation {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
        }
    }
    pub fn from_compile_time_env() -> Self {
        Self::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    }
}

impl Root {
    pub fn new(uri: &str) -> Self {
        Self {
            uri: uri.to_string(),
            name: None,
        }
    }
    pub fn with_name(mut self, name: impl Display) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn from_file_path(path: impl AsRef<Path>) -> Option<Self> {
        Some(Self::new(Url::from_file_path(path).ok()?.as_str()))
    }
    pub fn to_file_path(&self) -> Option<PathBuf> {
        Url::from_str(&self.uri).ok()?.to_file_path().ok()
    }
}

impl From<Vec<Root>> for ListRootsResult {
    fn from(roots: Vec<Root>) -> Self {
        ListRootsResult {
            roots,
            meta: Default::default(),
        }
    }
}

impl From<CompleteResultCompletion> for CompleteResult {
    fn from(completion: CompleteResultCompletion) -> Self {
        Self {
            completion,
            meta: Default::default(),
        }
    }
}
impl CompleteResultCompletion {
    pub const MAX_VALUES: usize = 100;
}

impl From<Vec<String>> for CompleteResultCompletion {
    fn from(mut values: Vec<String>) -> Self {
        let total = Some(values.len() as i64);
        let has_more = if values.len() > Self::MAX_VALUES {
            values.truncate(Self::MAX_VALUES);
            Some(true)
        } else {
            None
        };
        Self {
            has_more,
            total,
            values,
        }
    }
}
impl From<&[&str]> for CompleteResultCompletion {
    fn from(values: &[&str]) -> Self {
        values
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .into()
    }
}

impl CompleteRequestParams {
    pub fn new(r: CompleteRequestParamsRef, argument: CompleteRequestParamsArgument) -> Self {
        Self { argument, ref_: r }
    }
}
impl CompleteRequestParamsArgument {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

impl CompleteRequestParamsRef {
    pub fn new_prompt(name: &str) -> Self {
        CompleteRequestParamsRef::PromptReference(PromptReference::new(name))
    }
    pub fn new_resource(uri: &str) -> Self {
        CompleteRequestParamsRef::ResourceReference(ResourceReference::new(uri))
    }
}
impl PromptReference {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            type_: "ref/prompt".to_string(),
        }
    }
}
impl ResourceReference {
    pub fn new(uri: &str) -> Self {
        Self {
            uri: uri.to_string(),
            type_: "ref/resource".to_string(),
        }
    }
}
