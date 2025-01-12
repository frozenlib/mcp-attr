//! Model Context Protocol Types
//!
//! This code is automatically generated from [Model Context Protocol Schema](https://github.com/modelcontextprotocol/specification/blob/main/schema/schema.json).
//!
//! Schema: <http://json-schema.org/draft-07/schema#>

#![allow(rustdoc::bare_urls)]
use crate::utils::Integer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UnsubscribeRequestParams {
    /// The URI of the resource to unsubscribe from.
    #[serde(rename = "uri")]
    pub r#uri: String,
}
impl UnsubscribeRequestParams {
    pub fn new(r#uri: String) -> Self {
        Self { r#uri }
    }
}
/// Sent from the client to request cancellation of resources/updated notifications from the server. This should follow a previous resources/subscribe request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UnsubscribeRequest {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: UnsubscribeRequestParams,
}
impl UnsubscribeRequest {
    pub const METHOD: &str = "resources/unsubscribe";
    pub fn new(r#params: UnsubscribeRequestParams) -> Self {
        Self {
            r#method: "resources/unsubscribe".to_string(),
            r#params,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LoggingMessageNotificationParams {
    /// The severity of this log message.
    #[serde(rename = "level")]
    pub r#level: LoggingLevel,
    /// An optional name of the logger issuing this message.
    #[serde(rename = "logger")]
    pub r#logger: Option<String>,
    /// The data to be logged, such as a string message or an object. Any JSON serializable type is allowed here.
    #[serde(rename = "data")]
    pub r#data: serde_json::Value,
}
impl LoggingMessageNotificationParams {
    pub fn new(r#level: LoggingLevel, r#data: serde_json::Value) -> Self {
        Self {
            r#level,
            r#logger: None,
            r#data,
        }
    }
}
/// Notification of a log message passed from server to client. If no logging/setLevel request has been sent from the client, the server MAY decide which messages to send automatically.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LoggingMessageNotification {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: LoggingMessageNotificationParams,
}
impl LoggingMessageNotification {
    pub const METHOD: &str = "notifications/message";
    pub fn new(r#params: LoggingMessageNotificationParams) -> Self {
        Self {
            r#method: "notifications/message".to_string(),
            r#params,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreateMessageResultContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
}
/// The client's response to a sampling/create_message request from the server. The client should inform the user before returning the sampled message, to allow them to inspect the response (human in the loop) and decide whether to allow the server to see it.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateMessageResult {
    /// The reason why sampling stopped, if known.
    #[serde(rename = "stopReason")]
    pub r#stop_reason: Option<String>,
    #[serde(rename = "content")]
    pub r#content: CreateMessageResultContent,
    /// This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    #[serde(rename = "role")]
    pub r#role: Role,
    /// The name of the model that generated the message.
    #[serde(rename = "model")]
    pub r#model: String,
}
impl CreateMessageResult {
    pub fn new(r#content: CreateMessageResultContent, r#role: Role, r#model: String) -> Self {
        Self {
            r#stop_reason: None,
            r#content,
            r#meta: None,
            r#role,
            r#model,
        }
    }
}
/// The argument's information
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CompleteRequestParamsArgument {
    /// The name of the argument
    #[serde(rename = "name")]
    pub r#name: String,
    /// The value of the argument to use for completion matching.
    #[serde(rename = "value")]
    pub r#value: String,
}
impl CompleteRequestParamsArgument {
    pub fn new(r#name: String, r#value: String) -> Self {
        Self { r#name, r#value }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CompleteRequestParamsRef {
    PromptReference(PromptReference),
    ResourceReference(ResourceReference),
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CompleteRequestParams {
    /// The argument's information
    #[serde(rename = "argument")]
    pub r#argument: CompleteRequestParamsArgument,
    #[serde(rename = "ref")]
    pub r#ref: CompleteRequestParamsRef,
}
impl CompleteRequestParams {
    pub fn new(r#argument: CompleteRequestParamsArgument, r#ref: CompleteRequestParamsRef) -> Self {
        Self { r#argument, r#ref }
    }
}
/// A request from the client to the server, to ask for completion options.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CompleteRequest {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: CompleteRequestParams,
}
impl CompleteRequest {
    pub const METHOD: &str = "completion/complete";
    pub fn new(r#params: CompleteRequestParams) -> Self {
        Self {
            r#method: "completion/complete".to_string(),
            r#params,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PromptMessageContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
    EmbeddedResource(EmbeddedResource),
}
/// Describes a message returned as part of a prompt.
///
/// This is similar to `SamplingMessage`, but also supports the embedding of
/// resources from the MCP server.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PromptMessage {
    #[serde(rename = "content")]
    pub r#content: PromptMessageContent,
    #[serde(rename = "role")]
    pub r#role: Role,
}
impl PromptMessage {
    pub fn new(r#content: PromptMessageContent, r#role: Role) -> Self {
        Self { r#content, r#role }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InitializeRequestParams {
    #[serde(rename = "capabilities")]
    pub r#capabilities: ClientCapabilities,
    /// The latest version of the Model Context Protocol that the client supports. The client MAY decide to support older versions as well.
    #[serde(rename = "protocolVersion")]
    pub r#protocol_version: String,
    #[serde(rename = "clientInfo")]
    pub r#client_info: Implementation,
}
impl InitializeRequestParams {
    pub fn new(
        r#capabilities: ClientCapabilities,
        r#protocol_version: String,
        r#client_info: Implementation,
    ) -> Self {
        Self {
            r#capabilities,
            r#protocol_version,
            r#client_info,
        }
    }
}
/// This request is sent from the client to the server when it first connects, asking it to begin initialization.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InitializeRequest {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: InitializeRequestParams,
}
impl InitializeRequest {
    pub const METHOD: &str = "initialize";
    pub fn new(r#params: InitializeRequestParams) -> Self {
        Self {
            r#method: "initialize".to_string(),
            r#params,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListToolsRequestParams {
    /// An opaque token representing the current pagination position.
    /// If provided, the server should return results starting after this cursor.
    #[serde(rename = "cursor")]
    pub r#cursor: Option<String>,
}
impl ListToolsRequestParams {
    pub fn new() -> Self {
        Self { r#cursor: None }
    }
}
/// Sent from the client to request a list of tools the server has.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListToolsRequest {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: Option<ListToolsRequestParams>,
}
impl ListToolsRequest {
    pub const METHOD: &str = "tools/list";
    pub fn new() -> Self {
        Self {
            r#method: "tools/list".to_string(),
            r#params: None,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RequestParamsMeta {
    /// If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.
    #[serde(rename = "progressToken")]
    pub r#progress_token: Option<ProgressToken>,
}
impl RequestParamsMeta {
    pub fn new() -> Self {
        Self {
            r#progress_token: None,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RequestParams {
    #[serde(rename = "_meta")]
    pub r#meta: Option<RequestParamsMeta>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl RequestParams {
    pub fn new() -> Self {
        Self {
            r#meta: None,
            additional_properties: HashMap::new(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Request {
    #[serde(rename = "params")]
    pub r#params: Option<RequestParams>,
    #[serde(rename = "method")]
    pub r#method: String,
}
impl Request {
    pub fn new(r#method: String) -> Self {
        Self {
            r#params: None,
            r#method,
        }
    }
}
/// The contents of a specific resource or sub-resource.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResourceContents {
    /// The URI of this resource.
    #[serde(rename = "uri")]
    pub r#uri: String,
    /// The MIME type of this resource, if known.
    #[serde(rename = "mimeType")]
    pub r#mime_type: Option<String>,
}
impl ResourceContents {
    pub fn new(r#uri: String) -> Self {
        Self {
            r#uri,
            r#mime_type: None,
        }
    }
}
/// Present if the server supports sending log messages to the client.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ServerCapabilitiesLogging {
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl ServerCapabilitiesLogging {
    pub fn new() -> Self {
        Self {
            additional_properties: HashMap::new(),
        }
    }
}
/// Present if the server offers any resources to read.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ServerCapabilitiesResources {
    /// Whether this server supports subscribing to resource updates.
    #[serde(rename = "subscribe")]
    pub r#subscribe: Option<bool>,
    /// Whether this server supports notifications for changes to the resource list.
    #[serde(rename = "listChanged")]
    pub r#list_changed: Option<bool>,
}
impl ServerCapabilitiesResources {
    pub fn new() -> Self {
        Self {
            r#subscribe: None,
            r#list_changed: None,
        }
    }
}
/// Present if the server offers any tools to call.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ServerCapabilitiesTools {
    /// Whether this server supports notifications for changes to the tool list.
    #[serde(rename = "listChanged")]
    pub r#list_changed: Option<bool>,
}
impl ServerCapabilitiesTools {
    pub fn new() -> Self {
        Self {
            r#list_changed: None,
        }
    }
}
/// Present if the server offers any prompt templates.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ServerCapabilitiesPrompts {
    /// Whether this server supports notifications for changes to the prompt list.
    #[serde(rename = "listChanged")]
    pub r#list_changed: Option<bool>,
}
impl ServerCapabilitiesPrompts {
    pub fn new() -> Self {
        Self {
            r#list_changed: None,
        }
    }
}
/// Capabilities that a server may support. Known capabilities are defined here, in this schema, but this is not a closed set: any server can define its own, additional capabilities.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ServerCapabilities {
    /// Experimental, non-standard capabilities that the server supports.
    #[serde(rename = "experimental")]
    pub r#experimental: Option<serde_json::Value>,
    /// Present if the server supports sending log messages to the client.
    #[serde(rename = "logging")]
    pub r#logging: Option<ServerCapabilitiesLogging>,
    /// Present if the server offers any resources to read.
    #[serde(rename = "resources")]
    pub r#resources: Option<ServerCapabilitiesResources>,
    /// Present if the server offers any tools to call.
    #[serde(rename = "tools")]
    pub r#tools: Option<ServerCapabilitiesTools>,
    /// Present if the server offers any prompt templates.
    #[serde(rename = "prompts")]
    pub r#prompts: Option<ServerCapabilitiesPrompts>,
}
impl ServerCapabilities {
    pub fn new() -> Self {
        Self {
            r#experimental: None,
            r#logging: None,
            r#resources: None,
            r#tools: None,
            r#prompts: None,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListPromptsRequestParams {
    /// An opaque token representing the current pagination position.
    /// If provided, the server should return results starting after this cursor.
    #[serde(rename = "cursor")]
    pub r#cursor: Option<String>,
}
impl ListPromptsRequestParams {
    pub fn new() -> Self {
        Self { r#cursor: None }
    }
}
/// Sent from the client to request a list of prompts and prompt templates the server has.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListPromptsRequest {
    #[serde(rename = "params")]
    pub r#params: Option<ListPromptsRequestParams>,
    #[serde(rename = "method")]
    pub r#method: String,
}
impl ListPromptsRequest {
    pub const METHOD: &str = "prompts/list";
    pub fn new() -> Self {
        Self {
            r#params: None,
            r#method: "prompts/list".to_string(),
        }
    }
}
/// Optional metadata to pass through to the LLM provider. The format of this metadata is provider-specific.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateMessageRequestParamsMetadata {
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl CreateMessageRequestParamsMetadata {
    pub fn new() -> Self {
        Self {
            additional_properties: HashMap::new(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateMessageRequestParams {
    #[serde(rename = "temperature")]
    pub r#temperature: Option<f64>,
    #[serde(rename = "messages")]
    pub r#messages: Vec<SamplingMessage>,
    /// An optional system prompt the server wants to use for sampling. The client MAY modify or omit this prompt.
    #[serde(rename = "systemPrompt")]
    pub r#system_prompt: Option<String>,
    /// The server's preferences for which model to select. The client MAY ignore these preferences.
    #[serde(rename = "modelPreferences")]
    pub r#model_preferences: Option<ModelPreferences>,
    /// A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request.
    #[serde(rename = "includeContext")]
    pub r#include_context: Option<String>,
    /// The maximum number of tokens to sample, as requested by the server. The client MAY choose to sample fewer tokens than requested.
    #[serde(rename = "maxTokens")]
    pub r#max_tokens: Integer,
    #[serde(rename = "stopSequences")]
    pub r#stop_sequences: Option<Vec<String>>,
    /// Optional metadata to pass through to the LLM provider. The format of this metadata is provider-specific.
    #[serde(rename = "metadata")]
    pub r#metadata: Option<CreateMessageRequestParamsMetadata>,
}
impl CreateMessageRequestParams {
    pub fn new(r#messages: Vec<SamplingMessage>, r#max_tokens: Integer) -> Self {
        Self {
            r#temperature: None,
            r#messages,
            r#system_prompt: None,
            r#model_preferences: None,
            r#include_context: None,
            r#max_tokens,
            r#stop_sequences: None,
            r#metadata: None,
        }
    }
}
/// A request from the server to sample an LLM via the client. The client has full discretion over which model to select. The client should also inform the user before beginning sampling, to allow them to inspect the request (human in the loop) and decide whether to approve it.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateMessageRequest {
    #[serde(rename = "params")]
    pub r#params: CreateMessageRequestParams,
    #[serde(rename = "method")]
    pub r#method: String,
}
impl CreateMessageRequest {
    pub const METHOD: &str = "sampling/createMessage";
    pub fn new(r#params: CreateMessageRequestParams) -> Self {
        Self {
            r#params,
            r#method: "sampling/createMessage".to_string(),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ToolListChangedNotificationParams {
    /// This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl ToolListChangedNotificationParams {
    pub fn new() -> Self {
        Self {
            r#meta: None,
            additional_properties: HashMap::new(),
        }
    }
}
/// An optional notification from the server to the client, informing it that the list of tools it offers has changed. This may be issued by servers without any previous subscription from the client.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ToolListChangedNotification {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: Option<ToolListChangedNotificationParams>,
}
impl ToolListChangedNotification {
    pub const METHOD: &str = "notifications/tools/list_changed";
    pub fn new() -> Self {
        Self {
            r#method: "notifications/tools/list_changed".to_string(),
            r#params: None,
        }
    }
}
/// A prompt or prompt template that the server offers.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Prompt {
    /// An optional description of what this prompt provides
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// A list of arguments to use for templating the prompt.
    #[serde(rename = "arguments")]
    pub r#arguments: Option<Vec<PromptArgument>>,
    /// The name of the prompt or prompt template.
    #[serde(rename = "name")]
    pub r#name: String,
}
impl Prompt {
    pub fn new(r#name: String) -> Self {
        Self {
            r#description: None,
            r#arguments: None,
            r#name,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct NotificationParams {
    /// This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl NotificationParams {
    pub fn new() -> Self {
        Self {
            r#meta: None,
            additional_properties: HashMap::new(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Notification {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: Option<NotificationParams>,
}
impl Notification {
    pub fn new(r#method: String) -> Self {
        Self {
            r#method,
            r#params: None,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RootsListChangedNotificationParams {
    /// This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl RootsListChangedNotificationParams {
    pub fn new() -> Self {
        Self {
            r#meta: None,
            additional_properties: HashMap::new(),
        }
    }
}
/// A notification from the client to the server, informing it that the list of roots has changed.
/// This notification should be sent whenever the client adds, removes, or modifies any root.
/// The server should then request an updated list of roots using the ListRootsRequest.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RootsListChangedNotification {
    #[serde(rename = "params")]
    pub r#params: Option<RootsListChangedNotificationParams>,
    #[serde(rename = "method")]
    pub r#method: String,
}
impl RootsListChangedNotification {
    pub const METHOD: &str = "notifications/roots/list_changed";
    pub fn new() -> Self {
        Self {
            r#params: None,
            r#method: "notifications/roots/list_changed".to_string(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ReadResourceRequestParams {
    /// The URI of the resource to read. The URI can use any protocol; it is up to the server how to interpret it.
    #[serde(rename = "uri")]
    pub r#uri: String,
}
impl ReadResourceRequestParams {
    pub fn new(r#uri: String) -> Self {
        Self { r#uri }
    }
}
/// Sent from the client to the server, to read a specific resource URI.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ReadResourceRequest {
    #[serde(rename = "params")]
    pub r#params: ReadResourceRequestParams,
    #[serde(rename = "method")]
    pub r#method: String,
}
impl ReadResourceRequest {
    pub const METHOD: &str = "resources/read";
    pub fn new(r#params: ReadResourceRequestParams) -> Self {
        Self {
            r#params,
            r#method: "resources/read".to_string(),
        }
    }
}
/// The server's response to a tools/list request from the client.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListToolsResult {
    /// This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    /// An opaque token representing the pagination position after the last returned result.
    /// If present, there may be more results available.
    #[serde(rename = "nextCursor")]
    pub r#next_cursor: Option<String>,
    #[serde(rename = "tools")]
    pub r#tools: Vec<Tool>,
}
impl ListToolsResult {
    pub fn new(r#tools: Vec<Tool>) -> Self {
        Self {
            r#meta: None,
            r#next_cursor: None,
            r#tools,
        }
    }
}
/// An opaque token used to represent a cursor for pagination.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Cursor {
    String(String),
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PromptListChangedNotificationParams {
    /// This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl PromptListChangedNotificationParams {
    pub fn new() -> Self {
        Self {
            r#meta: None,
            additional_properties: HashMap::new(),
        }
    }
}
/// An optional notification from the server to the client, informing it that the list of prompts it offers has changed. This may be issued by servers without any previous subscription from the client.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PromptListChangedNotification {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: Option<PromptListChangedNotificationParams>,
}
impl PromptListChangedNotification {
    pub const METHOD: &str = "notifications/prompts/list_changed";
    pub fn new() -> Self {
        Self {
            r#method: "notifications/prompts/list_changed".to_string(),
            r#params: None,
        }
    }
}
/// Identifies a prompt.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PromptReference {
    #[serde(rename = "type")]
    pub r#type: String,
    /// The name of the prompt or prompt template
    #[serde(rename = "name")]
    pub r#name: String,
}
impl PromptReference {
    pub const TYPE: &str = "ref/prompt";
    pub fn new(r#name: String) -> Self {
        Self {
            r#type: "ref/prompt".to_string(),
            r#name,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SetLevelRequestParams {
    /// The level of logging that the client wants to receive from the server. The server should send all logs at this level and higher (i.e., more severe) to the client as notifications/logging/message.
    #[serde(rename = "level")]
    pub r#level: LoggingLevel,
}
impl SetLevelRequestParams {
    pub fn new(r#level: LoggingLevel) -> Self {
        Self { r#level }
    }
}
/// A request from the client to the server, to enable or adjust logging.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SetLevelRequest {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: SetLevelRequestParams,
}
impl SetLevelRequest {
    pub const METHOD: &str = "logging/setLevel";
    pub fn new(r#params: SetLevelRequestParams) -> Self {
        Self {
            r#method: "logging/setLevel".to_string(),
            r#params,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ServerRequest {
    PingRequest(PingRequest),
    CreateMessageRequest(CreateMessageRequest),
    ListRootsRequest(ListRootsRequest),
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetPromptRequestParams {
    /// Arguments to use for templating the prompt.
    #[serde(rename = "arguments")]
    pub r#arguments: Option<serde_json::Value>,
    /// The name of the prompt or prompt template.
    #[serde(rename = "name")]
    pub r#name: String,
}
impl GetPromptRequestParams {
    pub fn new(r#name: String) -> Self {
        Self {
            r#arguments: None,
            r#name,
        }
    }
}
/// Used by the client to get a prompt provided by the server.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetPromptRequest {
    #[serde(rename = "params")]
    pub r#params: GetPromptRequestParams,
    #[serde(rename = "method")]
    pub r#method: String,
}
impl GetPromptRequest {
    pub const METHOD: &str = "prompts/get";
    pub fn new(r#params: GetPromptRequestParams) -> Self {
        Self {
            r#params,
            r#method: "prompts/get".to_string(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CancelledNotificationParams {
    /// The ID of the request to cancel.
    ///
    /// This MUST correspond to the ID of a request previously issued in the same direction.
    #[serde(rename = "requestId")]
    pub r#request_id: RequestId,
    /// An optional string describing the reason for the cancellation. This MAY be logged or presented to the user.
    #[serde(rename = "reason")]
    pub r#reason: Option<String>,
}
impl CancelledNotificationParams {
    pub fn new(r#request_id: RequestId) -> Self {
        Self {
            r#request_id,
            r#reason: None,
        }
    }
}
/// This notification can be sent by either side to indicate that it is cancelling a previously-issued request.
///
/// The request SHOULD still be in-flight, but due to communication latency, it is always possible that this notification MAY arrive after the request has already finished.
///
/// This notification indicates that the result will be unused, so any associated processing SHOULD cease.
///
/// A client MUST NOT attempt to cancel its `initialize` request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CancelledNotification {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: CancelledNotificationParams,
}
impl CancelledNotification {
    pub const METHOD: &str = "notifications/cancelled";
    pub fn new(r#params: CancelledNotificationParams) -> Self {
        Self {
            r#method: "notifications/cancelled".to_string(),
            r#params,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ImageContentAnnotations {
    /// Describes how important this data is for operating the server.
    ///
    /// A value of 1 means "most important," and indicates that the data is
    /// effectively required, while 0 means "least important," and indicates that
    /// the data is entirely optional.
    #[serde(rename = "priority")]
    pub r#priority: Option<f64>,
    /// Describes who the intended customer of this object or data is.
    ///
    /// It can include multiple entries to indicate content useful for multiple audiences (e.g., `["user", "assistant"]`).
    #[serde(rename = "audience")]
    pub r#audience: Option<Vec<Role>>,
}
impl ImageContentAnnotations {
    pub fn new() -> Self {
        Self {
            r#priority: None,
            r#audience: None,
        }
    }
}
/// An image provided to or from an LLM.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ImageContent {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "annotations")]
    pub r#annotations: Option<ImageContentAnnotations>,
    /// The base64-encoded image data.
    #[serde(rename = "data")]
    pub r#data: String,
    /// The MIME type of the image. Different providers may support different image types.
    #[serde(rename = "mimeType")]
    pub r#mime_type: String,
}
impl ImageContent {
    pub const TYPE: &str = "image";
    pub fn new(r#data: String, r#mime_type: String) -> Self {
        Self {
            r#type: "image".to_string(),
            r#annotations: None,
            r#data,
            r#mime_type,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum JSONRPCMessage {
    JSONRPCRequest(JSONRPCRequest),
    JSONRPCNotification(JSONRPCNotification),
    JSONRPCResponse(JSONRPCResponse),
    JSONRPCError(JSONRPCError),
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ServerResult {
    Result(Result),
    InitializeResult(InitializeResult),
    ListResourcesResult(ListResourcesResult),
    ListResourceTemplatesResult(ListResourceTemplatesResult),
    ReadResourceResult(ReadResourceResult),
    ListPromptsResult(ListPromptsResult),
    GetPromptResult(GetPromptResult),
    ListToolsResult(ListToolsResult),
    CallToolResult(CallToolResult),
    CompleteResult(CompleteResult),
}
/// The server's response to a prompts/list request from the client.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListPromptsResult {
    /// This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    /// An opaque token representing the pagination position after the last returned result.
    /// If present, there may be more results available.
    #[serde(rename = "nextCursor")]
    pub r#next_cursor: Option<String>,
    #[serde(rename = "prompts")]
    pub r#prompts: Vec<Prompt>,
}
impl ListPromptsResult {
    pub fn new(r#prompts: Vec<Prompt>) -> Self {
        Self {
            r#meta: None,
            r#next_cursor: None,
            r#prompts,
        }
    }
}
/// Describes an argument that a prompt can accept.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PromptArgument {
    /// A human-readable description of the argument.
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Whether this argument must be provided.
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
    /// The name of the argument.
    #[serde(rename = "name")]
    pub r#name: String,
}
impl PromptArgument {
    pub fn new(r#name: String) -> Self {
        Self {
            r#description: None,
            r#required: None,
            r#name,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CompleteResultCompletion {
    /// The total number of completion options available. This can exceed the number of values actually sent in the response.
    #[serde(rename = "total")]
    pub r#total: Option<Integer>,
    /// Indicates whether there are additional completion options beyond those provided in the current response, even if the exact total is unknown.
    #[serde(rename = "hasMore")]
    pub r#has_more: Option<bool>,
    /// An array of completion values. Must not exceed 100 items.
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
impl CompleteResultCompletion {
    pub fn new(r#values: Vec<String>) -> Self {
        Self {
            r#total: None,
            r#has_more: None,
            r#values,
        }
    }
}
/// The server's response to a completion/complete request
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CompleteResult {
    /// This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    #[serde(rename = "completion")]
    pub r#completion: CompleteResultCompletion,
}
impl CompleteResult {
    pub fn new(r#completion: CompleteResultCompletion) -> Self {
        Self {
            r#meta: None,
            r#completion,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListResourceTemplatesRequestParams {
    /// An opaque token representing the current pagination position.
    /// If provided, the server should return results starting after this cursor.
    #[serde(rename = "cursor")]
    pub r#cursor: Option<String>,
}
impl ListResourceTemplatesRequestParams {
    pub fn new() -> Self {
        Self { r#cursor: None }
    }
}
/// Sent from the client to request a list of resource templates the server has.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListResourceTemplatesRequest {
    #[serde(rename = "params")]
    pub r#params: Option<ListResourceTemplatesRequestParams>,
    #[serde(rename = "method")]
    pub r#method: String,
}
impl ListResourceTemplatesRequest {
    pub const METHOD: &str = "resources/templates/list";
    pub fn new() -> Self {
        Self {
            r#params: None,
            r#method: "resources/templates/list".to_string(),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListRootsRequestParamsMeta {
    /// If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.
    #[serde(rename = "progressToken")]
    pub r#progress_token: Option<ProgressToken>,
}
impl ListRootsRequestParamsMeta {
    pub fn new() -> Self {
        Self {
            r#progress_token: None,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListRootsRequestParams {
    #[serde(rename = "_meta")]
    pub r#meta: Option<ListRootsRequestParamsMeta>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl ListRootsRequestParams {
    pub fn new() -> Self {
        Self {
            r#meta: None,
            additional_properties: HashMap::new(),
        }
    }
}
/// Sent from the server to request a list of root URIs from the client. Roots allow
/// servers to ask for specific directories or files to operate on. A common example
/// for roots is providing a set of repositories or directories a server should operate
/// on.
///
/// This request is typically used when the server needs to understand the file system
/// structure or access specific locations that the client has permission to read from.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListRootsRequest {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: Option<ListRootsRequestParams>,
}
impl ListRootsRequest {
    pub const METHOD: &str = "roots/list";
    pub fn new() -> Self {
        Self {
            r#method: "roots/list".to_string(),
            r#params: None,
        }
    }
}
/// The server's preferences for model selection, requested of the client during sampling.
///
/// Because LLMs can vary along multiple dimensions, choosing the "best" model is
/// rarely straightforward.  Different models excel in different areas—some are
/// faster but less capable, others are more capable but more expensive, and so
/// on. This interface allows servers to express their priorities across multiple
/// dimensions to help clients make an appropriate selection for their use case.
///
/// These preferences are always advisory. The client MAY ignore them. It is also
/// up to the client to decide how to interpret these preferences and how to
/// balance them against other considerations.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ModelPreferences {
    /// How much to prioritize cost when selecting a model. A value of 0 means cost
    /// is not important, while a value of 1 means cost is the most important
    /// factor.
    #[serde(rename = "costPriority")]
    pub r#cost_priority: Option<f64>,
    /// Optional hints to use for model selection.
    ///
    /// If multiple hints are specified, the client MUST evaluate them in order
    /// (such that the first match is taken).
    ///
    /// The client SHOULD prioritize these hints over the numeric priorities, but
    /// MAY still use the priorities to select from ambiguous matches.
    #[serde(rename = "hints")]
    pub r#hints: Option<Vec<ModelHint>>,
    /// How much to prioritize intelligence and capabilities when selecting a
    /// model. A value of 0 means intelligence is not important, while a value of 1
    /// means intelligence is the most important factor.
    #[serde(rename = "intelligencePriority")]
    pub r#intelligence_priority: Option<f64>,
    /// How much to prioritize sampling speed (latency) when selecting a model. A
    /// value of 0 means speed is not important, while a value of 1 means speed is
    /// the most important factor.
    #[serde(rename = "speedPriority")]
    pub r#speed_priority: Option<f64>,
}
impl ModelPreferences {
    pub fn new() -> Self {
        Self {
            r#cost_priority: None,
            r#hints: None,
            r#intelligence_priority: None,
            r#speed_priority: None,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PaginatedResult {
    /// An opaque token representing the pagination position after the last returned result.
    /// If present, there may be more results available.
    #[serde(rename = "nextCursor")]
    pub r#next_cursor: Option<String>,
    /// This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
}
impl PaginatedResult {
    pub fn new() -> Self {
        Self {
            r#next_cursor: None,
            r#meta: None,
        }
    }
}
/// A reference to a resource or resource template definition.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResourceReference {
    #[serde(rename = "type")]
    pub r#type: String,
    /// The URI or URI template of the resource.
    #[serde(rename = "uri")]
    pub r#uri: String,
}
impl ResourceReference {
    pub const TYPE: &str = "ref/resource";
    pub fn new(r#uri: String) -> Self {
        Self {
            r#type: "ref/resource".to_string(),
            r#uri,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Result {
    /// This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl Result {
    pub fn new() -> Self {
        Self {
            r#meta: None,
            additional_properties: HashMap::new(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SamplingMessageContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
}
/// Describes a message issued to or received from an LLM API.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SamplingMessage {
    #[serde(rename = "content")]
    pub r#content: SamplingMessageContent,
    #[serde(rename = "role")]
    pub r#role: Role,
}
impl SamplingMessage {
    pub fn new(r#content: SamplingMessageContent, r#role: Role) -> Self {
        Self { r#content, r#role }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CallToolResultContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
    EmbeddedResource(EmbeddedResource),
}
/// The server's response to a tool call.
///
/// Any errors that originate from the tool SHOULD be reported inside the result
/// object, with `isError` set to true, _not_ as an MCP protocol-level error
/// response. Otherwise, the LLM would not be able to see that an error occurred
/// and self-correct.
///
/// However, any errors in _finding_ the tool, an error indicating that the
/// server does not support tool calls, or any other exceptional conditions,
/// should be reported as an MCP error response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CallToolResult {
    /// This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    /// Whether the tool call ended in an error.
    ///
    /// If not set, this is assumed to be false (the call was successful).
    #[serde(rename = "isError")]
    pub r#is_error: Option<bool>,
    #[serde(rename = "content")]
    pub r#content: Vec<CallToolResultContent>,
}
impl CallToolResult {
    pub fn new(r#content: Vec<CallToolResultContent>) -> Self {
        Self {
            r#meta: None,
            r#is_error: None,
            r#content,
        }
    }
}
/// A uniquely identifying ID for a request in JSON-RPC.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum RequestId {
    String(String),
    Integer(Integer),
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SubscribeRequestParams {
    /// The URI of the resource to subscribe to. The URI can use any protocol; it is up to the server how to interpret it.
    #[serde(rename = "uri")]
    pub r#uri: String,
}
impl SubscribeRequestParams {
    pub fn new(r#uri: String) -> Self {
        Self { r#uri }
    }
}
/// Sent from the client to request resources/updated notifications from the server whenever a particular resource changes.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SubscribeRequest {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: SubscribeRequestParams,
}
impl SubscribeRequest {
    pub const METHOD: &str = "resources/subscribe";
    pub fn new(r#params: SubscribeRequestParams) -> Self {
        Self {
            r#method: "resources/subscribe".to_string(),
            r#params,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResourceTemplateAnnotations {
    /// Describes how important this data is for operating the server.
    ///
    /// A value of 1 means "most important," and indicates that the data is
    /// effectively required, while 0 means "least important," and indicates that
    /// the data is entirely optional.
    #[serde(rename = "priority")]
    pub r#priority: Option<f64>,
    /// Describes who the intended customer of this object or data is.
    ///
    /// It can include multiple entries to indicate content useful for multiple audiences (e.g., `["user", "assistant"]`).
    #[serde(rename = "audience")]
    pub r#audience: Option<Vec<Role>>,
}
impl ResourceTemplateAnnotations {
    pub fn new() -> Self {
        Self {
            r#priority: None,
            r#audience: None,
        }
    }
}
/// A template description for resources available on the server.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResourceTemplate {
    #[serde(rename = "annotations")]
    pub r#annotations: Option<ResourceTemplateAnnotations>,
    /// A description of what this template is for.
    ///
    /// This can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a "hint" to the model.
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// A URI template (according to RFC 6570) that can be used to construct resource URIs.
    #[serde(rename = "uriTemplate")]
    pub r#uri_template: String,
    /// The MIME type for all resources that match this template. This should only be included if all resources matching this template have the same type.
    #[serde(rename = "mimeType")]
    pub r#mime_type: Option<String>,
    /// A human-readable name for the type of resource this template refers to.
    ///
    /// This can be used by clients to populate UI elements.
    #[serde(rename = "name")]
    pub r#name: String,
}
impl ResourceTemplate {
    pub fn new(r#uri_template: String, r#name: String) -> Self {
        Self {
            r#annotations: None,
            r#description: None,
            r#uri_template,
            r#mime_type: None,
            r#name,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TextContentAnnotations {
    /// Describes who the intended customer of this object or data is.
    ///
    /// It can include multiple entries to indicate content useful for multiple audiences (e.g., `["user", "assistant"]`).
    #[serde(rename = "audience")]
    pub r#audience: Option<Vec<Role>>,
    /// Describes how important this data is for operating the server.
    ///
    /// A value of 1 means "most important," and indicates that the data is
    /// effectively required, while 0 means "least important," and indicates that
    /// the data is entirely optional.
    #[serde(rename = "priority")]
    pub r#priority: Option<f64>,
}
impl TextContentAnnotations {
    pub fn new() -> Self {
        Self {
            r#audience: None,
            r#priority: None,
        }
    }
}
/// Text provided to or from an LLM.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TextContent {
    /// The text content of the message.
    #[serde(rename = "text")]
    pub r#text: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "annotations")]
    pub r#annotations: Option<TextContentAnnotations>,
}
impl TextContent {
    pub const TYPE: &str = "text";
    pub fn new(r#text: String) -> Self {
        Self {
            r#text,
            r#type: "text".to_string(),
            r#annotations: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BlobResourceContents {
    /// The MIME type of this resource, if known.
    #[serde(rename = "mimeType")]
    pub r#mime_type: Option<String>,
    /// A base64-encoded string representing the binary data of the item.
    #[serde(rename = "blob")]
    pub r#blob: String,
    /// The URI of this resource.
    #[serde(rename = "uri")]
    pub r#uri: String,
}
impl BlobResourceContents {
    pub fn new(r#blob: String, r#uri: String) -> Self {
        Self {
            r#mime_type: None,
            r#blob,
            r#uri,
        }
    }
}
/// The severity of a log message.
///
/// These map to syslog message severities, as specified in RFC-5424:
/// https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.1
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum LoggingLevel {
    String(String),
}
/// After receiving an initialize request from the client, the server sends this response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InitializeResult {
    #[serde(rename = "serverInfo")]
    pub r#server_info: Implementation,
    #[serde(rename = "capabilities")]
    pub r#capabilities: ServerCapabilities,
    /// Instructions describing how to use the server and its features.
    ///
    /// This can be used by clients to improve the LLM's understanding of available tools, resources, etc. It can be thought of like a "hint" to the model. For example, this information MAY be added to the system prompt.
    #[serde(rename = "instructions")]
    pub r#instructions: Option<String>,
    /// The version of the Model Context Protocol that the server wants to use. This may not match the version that the client requested. If the client cannot support this version, it MUST disconnect.
    #[serde(rename = "protocolVersion")]
    pub r#protocol_version: String,
    /// This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
}
impl InitializeResult {
    pub fn new(
        r#server_info: Implementation,
        r#capabilities: ServerCapabilities,
        r#protocol_version: String,
    ) -> Self {
        Self {
            r#server_info,
            r#capabilities,
            r#instructions: None,
            r#protocol_version,
            r#meta: None,
        }
    }
}
/// Represents a root directory or file that the server can operate on.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    /// An optional name for the root. This can be used to provide a human-readable
    /// identifier for the root, which may be useful for display purposes or for
    /// referencing the root in other parts of the application.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The URI identifying the root. This *must* start with file:// for now.
    /// This restriction may be relaxed in future versions of the protocol to allow
    /// other URI schemes.
    #[serde(rename = "uri")]
    pub r#uri: String,
}
impl Root {
    pub fn new(r#uri: String) -> Self {
        Self {
            r#name: None,
            r#uri,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ClientResult {
    Result(Result),
    CreateMessageResult(CreateMessageResult),
    ListRootsResult(ListRootsResult),
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ProgressNotificationParams {
    /// Total number of items to process (or total progress required), if known.
    #[serde(rename = "total")]
    pub r#total: Option<f64>,
    /// The progress thus far. This should increase every time progress is made, even if the total is unknown.
    #[serde(rename = "progress")]
    pub r#progress: f64,
    /// The progress token which was given in the initial request, used to associate this notification with the request that is proceeding.
    #[serde(rename = "progressToken")]
    pub r#progress_token: ProgressToken,
}
impl ProgressNotificationParams {
    pub fn new(r#progress: f64, r#progress_token: ProgressToken) -> Self {
        Self {
            r#total: None,
            r#progress,
            r#progress_token,
        }
    }
}
/// An out-of-band notification used to inform the receiver of a progress update for a long-running request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ProgressNotification {
    #[serde(rename = "params")]
    pub r#params: ProgressNotificationParams,
    #[serde(rename = "method")]
    pub r#method: String,
}
impl ProgressNotification {
    pub const METHOD: &str = "notifications/progress";
    pub fn new(r#params: ProgressNotificationParams) -> Self {
        Self {
            r#params,
            r#method: "notifications/progress".to_string(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ReadResourceResultContents {
    TextResourceContents(TextResourceContents),
    BlobResourceContents(BlobResourceContents),
}
/// The server's response to a resources/read request from the client.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ReadResourceResult {
    #[serde(rename = "contents")]
    pub r#contents: Vec<ReadResourceResultContents>,
    /// This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
}
impl ReadResourceResult {
    pub fn new(r#contents: Vec<ReadResourceResultContents>) -> Self {
        Self {
            r#contents,
            r#meta: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ServerNotification {
    CancelledNotification(CancelledNotification),
    ProgressNotification(ProgressNotification),
    ResourceListChangedNotification(ResourceListChangedNotification),
    ResourceUpdatedNotification(ResourceUpdatedNotification),
    PromptListChangedNotification(PromptListChangedNotification),
    ToolListChangedNotification(ToolListChangedNotification),
    LoggingMessageNotification(LoggingMessageNotification),
}
/// A JSON Schema object defining the expected parameters for the tool.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ToolInputSchema {
    #[serde(rename = "required")]
    pub r#required: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "properties")]
    pub r#properties: Option<serde_json::Value>,
}
impl ToolInputSchema {
    pub const TYPE: &str = "object";
    pub fn new() -> Self {
        Self {
            r#required: None,
            r#type: "object".to_string(),
            r#properties: None,
        }
    }
}
/// Definition for a tool the client can call.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Tool {
    /// A human-readable description of the tool.
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// A JSON Schema object defining the expected parameters for the tool.
    #[serde(rename = "inputSchema")]
    pub r#input_schema: ToolInputSchema,
    /// The name of the tool.
    #[serde(rename = "name")]
    pub r#name: String,
}
impl Tool {
    pub fn new(r#input_schema: ToolInputSchema, r#name: String) -> Self {
        Self {
            r#description: None,
            r#input_schema,
            r#name,
        }
    }
}
/// The server's response to a prompts/get request from the client.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetPromptResult {
    /// An optional description for the prompt.
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    #[serde(rename = "messages")]
    pub r#messages: Vec<PromptMessage>,
}
impl GetPromptResult {
    pub fn new(r#messages: Vec<PromptMessage>) -> Self {
        Self {
            r#description: None,
            r#meta: None,
            r#messages,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ClientNotification {
    CancelledNotification(CancelledNotification),
    InitializedNotification(InitializedNotification),
    ProgressNotification(ProgressNotification),
    RootsListChangedNotification(RootsListChangedNotification),
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResourceAnnotations {
    /// Describes how important this data is for operating the server.
    ///
    /// A value of 1 means "most important," and indicates that the data is
    /// effectively required, while 0 means "least important," and indicates that
    /// the data is entirely optional.
    #[serde(rename = "priority")]
    pub r#priority: Option<f64>,
    /// Describes who the intended customer of this object or data is.
    ///
    /// It can include multiple entries to indicate content useful for multiple audiences (e.g., `["user", "assistant"]`).
    #[serde(rename = "audience")]
    pub r#audience: Option<Vec<Role>>,
}
impl ResourceAnnotations {
    pub fn new() -> Self {
        Self {
            r#priority: None,
            r#audience: None,
        }
    }
}
/// A known resource that the server is capable of reading.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Resource {
    /// A human-readable name for this resource.
    ///
    /// This can be used by clients to populate UI elements.
    #[serde(rename = "name")]
    pub r#name: String,
    /// A description of what this resource represents.
    ///
    /// This can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a "hint" to the model.
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The URI of this resource.
    #[serde(rename = "uri")]
    pub r#uri: String,
    /// The MIME type of this resource, if known.
    #[serde(rename = "mimeType")]
    pub r#mime_type: Option<String>,
    #[serde(rename = "annotations")]
    pub r#annotations: Option<ResourceAnnotations>,
}
impl Resource {
    pub fn new(r#name: String, r#uri: String) -> Self {
        Self {
            r#name,
            r#description: None,
            r#uri,
            r#mime_type: None,
            r#annotations: None,
        }
    }
}
/// A successful (non-error) response to a request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JSONRPCResponse {
    #[serde(rename = "jsonrpc")]
    pub r#jsonrpc: String,
    #[serde(rename = "result")]
    pub r#result: Result,
    #[serde(rename = "id")]
    pub r#id: RequestId,
}
impl JSONRPCResponse {
    pub const JSONRPC: &str = "2.0";
    pub fn new(r#result: Result, r#id: RequestId) -> Self {
        Self {
            r#jsonrpc: "2.0".to_string(),
            r#result,
            r#id,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EmbeddedResourceResource {
    TextResourceContents(TextResourceContents),
    BlobResourceContents(BlobResourceContents),
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EmbeddedResourceAnnotations {
    /// Describes who the intended customer of this object or data is.
    ///
    /// It can include multiple entries to indicate content useful for multiple audiences (e.g., `["user", "assistant"]`).
    #[serde(rename = "audience")]
    pub r#audience: Option<Vec<Role>>,
    /// Describes how important this data is for operating the server.
    ///
    /// A value of 1 means "most important," and indicates that the data is
    /// effectively required, while 0 means "least important," and indicates that
    /// the data is entirely optional.
    #[serde(rename = "priority")]
    pub r#priority: Option<f64>,
}
impl EmbeddedResourceAnnotations {
    pub fn new() -> Self {
        Self {
            r#audience: None,
            r#priority: None,
        }
    }
}
/// The contents of a resource, embedded into a prompt or tool call result.
///
/// It is up to the client how best to render embedded resources for the benefit
/// of the LLM and/or the user.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EmbeddedResource {
    #[serde(rename = "resource")]
    pub r#resource: EmbeddedResourceResource,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "annotations")]
    pub r#annotations: Option<EmbeddedResourceAnnotations>,
}
impl EmbeddedResource {
    pub const TYPE: &str = "resource";
    pub fn new(r#resource: EmbeddedResourceResource) -> Self {
        Self {
            r#resource,
            r#type: "resource".to_string(),
            r#annotations: None,
        }
    }
}
/// Present if the client supports sampling from an LLM.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClientCapabilitiesSampling {
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl ClientCapabilitiesSampling {
    pub fn new() -> Self {
        Self {
            additional_properties: HashMap::new(),
        }
    }
}
/// Present if the client supports listing roots.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClientCapabilitiesRoots {
    /// Whether the client supports notifications for changes to the roots list.
    #[serde(rename = "listChanged")]
    pub r#list_changed: Option<bool>,
}
impl ClientCapabilitiesRoots {
    pub fn new() -> Self {
        Self {
            r#list_changed: None,
        }
    }
}
/// Capabilities a client may support. Known capabilities are defined here, in this schema, but this is not a closed set: any client can define its own, additional capabilities.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClientCapabilities {
    /// Present if the client supports sampling from an LLM.
    #[serde(rename = "sampling")]
    pub r#sampling: Option<ClientCapabilitiesSampling>,
    /// Experimental, non-standard capabilities that the client supports.
    #[serde(rename = "experimental")]
    pub r#experimental: Option<serde_json::Value>,
    /// Present if the client supports listing roots.
    #[serde(rename = "roots")]
    pub r#roots: Option<ClientCapabilitiesRoots>,
}
impl ClientCapabilities {
    pub fn new() -> Self {
        Self {
            r#sampling: None,
            r#experimental: None,
            r#roots: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResourceUpdatedNotificationParams {
    /// The URI of the resource that has been updated. This might be a sub-resource of the one that the client actually subscribed to.
    #[serde(rename = "uri")]
    pub r#uri: String,
}
impl ResourceUpdatedNotificationParams {
    pub fn new(r#uri: String) -> Self {
        Self { r#uri }
    }
}
/// A notification from the server to the client, informing it that a resource has changed and may need to be read again. This should only be sent if the client previously sent a resources/subscribe request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResourceUpdatedNotification {
    #[serde(rename = "params")]
    pub r#params: ResourceUpdatedNotificationParams,
    #[serde(rename = "method")]
    pub r#method: String,
}
impl ResourceUpdatedNotification {
    pub const METHOD: &str = "notifications/resources/updated";
    pub fn new(r#params: ResourceUpdatedNotificationParams) -> Self {
        Self {
            r#params,
            r#method: "notifications/resources/updated".to_string(),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AnnotatedAnnotations {
    /// Describes how important this data is for operating the server.
    ///
    /// A value of 1 means "most important," and indicates that the data is
    /// effectively required, while 0 means "least important," and indicates that
    /// the data is entirely optional.
    #[serde(rename = "priority")]
    pub r#priority: Option<f64>,
    /// Describes who the intended customer of this object or data is.
    ///
    /// It can include multiple entries to indicate content useful for multiple audiences (e.g., `["user", "assistant"]`).
    #[serde(rename = "audience")]
    pub r#audience: Option<Vec<Role>>,
}
impl AnnotatedAnnotations {
    pub fn new() -> Self {
        Self {
            r#priority: None,
            r#audience: None,
        }
    }
}
/// Base for objects that include optional annotations for the client. The client can use annotations to inform how objects are used or displayed
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Annotated {
    #[serde(rename = "annotations")]
    pub r#annotations: Option<AnnotatedAnnotations>,
}
impl Annotated {
    pub fn new() -> Self {
        Self {
            r#annotations: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ClientRequest {
    InitializeRequest(InitializeRequest),
    PingRequest(PingRequest),
    ListResourcesRequest(ListResourcesRequest),
    ListResourceTemplatesRequest(ListResourceTemplatesRequest),
    ReadResourceRequest(ReadResourceRequest),
    SubscribeRequest(SubscribeRequest),
    UnsubscribeRequest(UnsubscribeRequest),
    ListPromptsRequest(ListPromptsRequest),
    GetPromptRequest(GetPromptRequest),
    ListToolsRequest(ListToolsRequest),
    CallToolRequest(CallToolRequest),
    SetLevelRequest(SetLevelRequest),
    CompleteRequest(CompleteRequest),
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PaginatedRequestParams {
    /// An opaque token representing the current pagination position.
    /// If provided, the server should return results starting after this cursor.
    #[serde(rename = "cursor")]
    pub r#cursor: Option<String>,
}
impl PaginatedRequestParams {
    pub fn new() -> Self {
        Self { r#cursor: None }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PaginatedRequest {
    #[serde(rename = "params")]
    pub r#params: Option<PaginatedRequestParams>,
    #[serde(rename = "method")]
    pub r#method: String,
}
impl PaginatedRequest {
    pub fn new(r#method: String) -> Self {
        Self {
            r#params: None,
            r#method,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CallToolRequestParams {
    #[serde(rename = "arguments")]
    pub r#arguments: Option<serde_json::Value>,
    #[serde(rename = "name")]
    pub r#name: String,
}
impl CallToolRequestParams {
    pub fn new(r#name: String) -> Self {
        Self {
            r#arguments: None,
            r#name,
        }
    }
}
/// Used by the client to invoke a tool provided by the server.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CallToolRequest {
    #[serde(rename = "params")]
    pub r#params: CallToolRequestParams,
    #[serde(rename = "method")]
    pub r#method: String,
}
impl CallToolRequest {
    pub const METHOD: &str = "tools/call";
    pub fn new(r#params: CallToolRequestParams) -> Self {
        Self {
            r#params,
            r#method: "tools/call".to_string(),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PingRequestParamsMeta {
    /// If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.
    #[serde(rename = "progressToken")]
    pub r#progress_token: Option<ProgressToken>,
}
impl PingRequestParamsMeta {
    pub fn new() -> Self {
        Self {
            r#progress_token: None,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PingRequestParams {
    #[serde(rename = "_meta")]
    pub r#meta: Option<PingRequestParamsMeta>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl PingRequestParams {
    pub fn new() -> Self {
        Self {
            r#meta: None,
            additional_properties: HashMap::new(),
        }
    }
}
/// A ping, issued by either the server or the client, to check that the other party is still alive. The receiver must promptly respond, or else may be disconnected.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PingRequest {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: Option<PingRequestParams>,
}
impl PingRequest {
    pub const METHOD: &str = "ping";
    pub fn new() -> Self {
        Self {
            r#method: "ping".to_string(),
            r#params: None,
        }
    }
}
/// The client's response to a roots/list request from the server.
/// This result contains an array of Root objects, each representing a root directory
/// or file that the server can operate on.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListRootsResult {
    /// This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    #[serde(rename = "roots")]
    pub r#roots: Vec<Root>,
}
impl ListRootsResult {
    pub fn new(r#roots: Vec<Root>) -> Self {
        Self {
            r#meta: None,
            r#roots,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JSONRPCRequestParamsMeta {
    /// If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.
    #[serde(rename = "progressToken")]
    pub r#progress_token: Option<ProgressToken>,
}
impl JSONRPCRequestParamsMeta {
    pub fn new() -> Self {
        Self {
            r#progress_token: None,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JSONRPCRequestParams {
    #[serde(rename = "_meta")]
    pub r#meta: Option<JSONRPCRequestParamsMeta>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl JSONRPCRequestParams {
    pub fn new() -> Self {
        Self {
            r#meta: None,
            additional_properties: HashMap::new(),
        }
    }
}
/// A request that expects a response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JSONRPCRequest {
    #[serde(rename = "id")]
    pub r#id: RequestId,
    #[serde(rename = "params")]
    pub r#params: Option<JSONRPCRequestParams>,
    #[serde(rename = "jsonrpc")]
    pub r#jsonrpc: String,
    #[serde(rename = "method")]
    pub r#method: String,
}
impl JSONRPCRequest {
    pub const JSONRPC: &str = "2.0";
    pub fn new(r#id: RequestId, r#method: String) -> Self {
        Self {
            r#id,
            r#params: None,
            r#jsonrpc: "2.0".to_string(),
            r#method,
        }
    }
}
/// The server's response to a resources/list request from the client.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListResourcesResult {
    /// An opaque token representing the pagination position after the last returned result.
    /// If present, there may be more results available.
    #[serde(rename = "nextCursor")]
    pub r#next_cursor: Option<String>,
    #[serde(rename = "resources")]
    pub r#resources: Vec<Resource>,
    /// This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
}
impl ListResourcesResult {
    pub fn new(r#resources: Vec<Resource>) -> Self {
        Self {
            r#next_cursor: None,
            r#resources,
            r#meta: None,
        }
    }
}
/// A progress token, used to associate progress notifications with the original request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ProgressToken {
    String(String),
    Integer(Integer),
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListResourcesRequestParams {
    /// An opaque token representing the current pagination position.
    /// If provided, the server should return results starting after this cursor.
    #[serde(rename = "cursor")]
    pub r#cursor: Option<String>,
}
impl ListResourcesRequestParams {
    pub fn new() -> Self {
        Self { r#cursor: None }
    }
}
/// Sent from the client to request a list of resources the server has.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListResourcesRequest {
    #[serde(rename = "params")]
    pub r#params: Option<ListResourcesRequestParams>,
    #[serde(rename = "method")]
    pub r#method: String,
}
impl ListResourcesRequest {
    pub const METHOD: &str = "resources/list";
    pub fn new() -> Self {
        Self {
            r#params: None,
            r#method: "resources/list".to_string(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TextResourceContents {
    /// The text of the item. This must only be set if the item can actually be represented as text (not binary data).
    #[serde(rename = "text")]
    pub r#text: String,
    /// The URI of this resource.
    #[serde(rename = "uri")]
    pub r#uri: String,
    /// The MIME type of this resource, if known.
    #[serde(rename = "mimeType")]
    pub r#mime_type: Option<String>,
}
impl TextResourceContents {
    pub fn new(r#text: String, r#uri: String) -> Self {
        Self {
            r#text,
            r#uri,
            r#mime_type: None,
        }
    }
}
/// Describes the name and version of an MCP implementation.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Implementation {
    #[serde(rename = "version")]
    pub r#version: String,
    #[serde(rename = "name")]
    pub r#name: String,
}
impl Implementation {
    pub fn new(r#version: String, r#name: String) -> Self {
        Self { r#version, r#name }
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JSONRPCErrorError {
    /// Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).
    #[serde(rename = "data")]
    pub r#data: Option<serde_json::Value>,
    /// A short description of the error. The message SHOULD be limited to a concise single sentence.
    #[serde(rename = "message")]
    pub r#message: String,
    /// The error type that occurred.
    #[serde(rename = "code")]
    pub r#code: Integer,
}
impl JSONRPCErrorError {
    pub fn new(r#message: String, r#code: Integer) -> Self {
        Self {
            r#data: None,
            r#message,
            r#code,
        }
    }
}
/// A response to a request that indicates an error occurred.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JSONRPCError {
    #[serde(rename = "error")]
    pub r#error: JSONRPCErrorError,
    #[serde(rename = "id")]
    pub r#id: RequestId,
    #[serde(rename = "jsonrpc")]
    pub r#jsonrpc: String,
}
impl JSONRPCError {
    pub const JSONRPC: &str = "2.0";
    pub fn new(r#error: JSONRPCErrorError, r#id: RequestId) -> Self {
        Self {
            r#error,
            r#id,
            r#jsonrpc: "2.0".to_string(),
        }
    }
}
/// The sender or recipient of messages and data in a conversation.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Role {
    String(String),
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JSONRPCNotificationParams {
    /// This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl JSONRPCNotificationParams {
    pub fn new() -> Self {
        Self {
            r#meta: None,
            additional_properties: HashMap::new(),
        }
    }
}
/// A notification which does not expect a response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JSONRPCNotification {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: Option<JSONRPCNotificationParams>,
    #[serde(rename = "jsonrpc")]
    pub r#jsonrpc: String,
}
impl JSONRPCNotification {
    pub const JSONRPC: &str = "2.0";
    pub fn new(r#method: String) -> Self {
        Self {
            r#method,
            r#params: None,
            r#jsonrpc: "2.0".to_string(),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResourceListChangedNotificationParams {
    /// This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl ResourceListChangedNotificationParams {
    pub fn new() -> Self {
        Self {
            r#meta: None,
            additional_properties: HashMap::new(),
        }
    }
}
/// An optional notification from the server to the client, informing it that the list of resources it can read from has changed. This may be issued by servers without any previous subscription from the client.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResourceListChangedNotification {
    #[serde(rename = "method")]
    pub r#method: String,
    #[serde(rename = "params")]
    pub r#params: Option<ResourceListChangedNotificationParams>,
}
impl ResourceListChangedNotification {
    pub const METHOD: &str = "notifications/resources/list_changed";
    pub fn new() -> Self {
        Self {
            r#method: "notifications/resources/list_changed".to_string(),
            r#params: None,
        }
    }
}
/// The server's response to a resources/templates/list request from the client.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListResourceTemplatesResult {
    /// This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    /// An opaque token representing the pagination position after the last returned result.
    /// If present, there may be more results available.
    #[serde(rename = "nextCursor")]
    pub r#next_cursor: Option<String>,
    #[serde(rename = "resourceTemplates")]
    pub r#resource_templates: Vec<ResourceTemplate>,
}
impl ListResourceTemplatesResult {
    pub fn new(r#resource_templates: Vec<ResourceTemplate>) -> Self {
        Self {
            r#meta: None,
            r#next_cursor: None,
            r#resource_templates,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InitializedNotificationParams {
    /// This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.
    #[serde(rename = "_meta")]
    pub r#meta: Option<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}
impl InitializedNotificationParams {
    pub fn new() -> Self {
        Self {
            r#meta: None,
            additional_properties: HashMap::new(),
        }
    }
}
/// This notification is sent from the client to the server after initialization has finished.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InitializedNotification {
    #[serde(rename = "params")]
    pub r#params: Option<InitializedNotificationParams>,
    #[serde(rename = "method")]
    pub r#method: String,
}
impl InitializedNotification {
    pub const METHOD: &str = "notifications/initialized";
    pub fn new() -> Self {
        Self {
            r#params: None,
            r#method: "notifications/initialized".to_string(),
        }
    }
}
/// Hints to use for model selection.
///
/// Keys not declared here are currently left unspecified by the spec and are up
/// to the client to interpret.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ModelHint {
    /// A hint for a model name.
    ///
    /// The client SHOULD treat this as a substring of a model name; for example:
    ///  - `claude-3-5-sonnet` should match `claude-3-5-sonnet-20241022`
    ///  - `sonnet` should match `claude-3-5-sonnet-20241022`, `claude-3-sonnet-20240229`, etc.
    ///  - `claude` should match any Claude model
    ///
    /// The client MAY also map the string to a different provider's model name or a different model family, as long as it fills a similar niche; for example:
    ///  - `gemini-1.5-flash` could match `claude-3-haiku-20240307`
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}
impl ModelHint {
    pub fn new() -> Self {
        Self { r#name: None }
    }
}
