use crate::schema::*;
use crate::utils::*;
impl ::std::convert::From<&CreateMessageResult> for ClientResult {
    fn from(value: &CreateMessageResult) -> Self {
        <ClientResult as ::std::convert::From<CreateMessageResult>>::from(
            <CreateMessageResult as ::std::convert::From<&CreateMessageResult>>::from(value),
        )
    }
}
impl ::std::convert::From<&JsonrpcError> for JsonrpcMessage {
    fn from(value: &JsonrpcError) -> Self {
        <JsonrpcMessage as ::std::convert::From<JsonrpcError>>::from(
            <JsonrpcError as ::std::convert::From<&JsonrpcError>>::from(value),
        )
    }
}
impl ::std::convert::From<&JsonrpcNotification> for JsonrpcMessage {
    fn from(value: &JsonrpcNotification) -> Self {
        <JsonrpcMessage as ::std::convert::From<JsonrpcNotification>>::from(
            <JsonrpcNotification as ::std::convert::From<&JsonrpcNotification>>::from(value),
        )
    }
}
impl ::std::convert::From<&ListPromptsResult> for ServerResult {
    fn from(value: &ListPromptsResult) -> Self {
        <ServerResult as ::std::convert::From<ListPromptsResult>>::from(
            <ListPromptsResult as ::std::convert::From<&ListPromptsResult>>::from(value),
        )
    }
}
impl ::std::convert::From<&TextResourceContents> for EmbeddedResourceResource {
    fn from(value: &TextResourceContents) -> Self {
        <EmbeddedResourceResource as ::std::convert::From<TextResourceContents>>::from(
            <TextResourceContents as ::std::convert::From<&TextResourceContents>>::from(value),
        )
    }
}
impl ::std::convert::From<&TextResourceContents> for ReadResourceResult {
    fn from(value: &TextResourceContents) -> Self {
        <ReadResourceResult as ::std::convert::From<ReadResourceResultContentsItem>>::from(
            <ReadResourceResultContentsItem as ::std::convert::From<&TextResourceContents>>::from(
                value,
            ),
        )
    }
}
impl ::std::convert::From<&TextResourceContents> for ReadResourceResultContentsItem {
    fn from(value: &TextResourceContents) -> Self {
        <ReadResourceResultContentsItem as ::std::convert::From<TextResourceContents>>::from(
            <TextResourceContents as ::std::convert::From<&TextResourceContents>>::from(value),
        )
    }
}
impl ::std::convert::From<&TextResourceContents> for ServerResult {
    fn from(value: &TextResourceContents) -> Self {
        <ServerResult as ::std::convert::From<ReadResourceResult>>::from(
            <ReadResourceResult as ::std::convert::From<&TextResourceContents>>::from(value),
        )
    }
}
impl ::std::convert::From<&PromptReference> for CompleteRequestParamsRef {
    fn from(value: &PromptReference) -> Self {
        <CompleteRequestParamsRef as ::std::convert::From<PromptReference>>::from(
            <PromptReference as ::std::convert::From<&PromptReference>>::from(value),
        )
    }
}
impl ::std::convert::From<EmptyResult> for ClientResult {
    fn from(value: EmptyResult) -> Self {
        <ClientResult as ::std::convert::From<Result>>::from(<Result as ::std::convert::From<
            EmptyResult,
        >>::from(value))
    }
}
impl ::std::convert::From<EmptyResult> for ServerResult {
    fn from(value: EmptyResult) -> Self {
        <ServerResult as ::std::convert::From<Result>>::from(<Result as ::std::convert::From<
            EmptyResult,
        >>::from(value))
    }
}
impl ::std::convert::From<&ToolListChangedNotification> for ServerNotification {
    fn from(value: &ToolListChangedNotification) -> Self {
        < ServerNotification as :: std :: convert :: From < ToolListChangedNotification >> :: from (< ToolListChangedNotification as :: std :: convert :: From < & ToolListChangedNotification >> :: from (value))
    }
}
impl ::std::convert::From<&ListPromptsRequest> for ClientRequest {
    fn from(value: &ListPromptsRequest) -> Self {
        <ClientRequest as ::std::convert::From<ListPromptsRequest>>::from(
            <ListPromptsRequest as ::std::convert::From<&ListPromptsRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<&CreateMessageRequest> for ServerRequest {
    fn from(value: &CreateMessageRequest) -> Self {
        <ServerRequest as ::std::convert::From<CreateMessageRequest>>::from(
            <CreateMessageRequest as ::std::convert::From<&CreateMessageRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<&EmbeddedResource> for PromptMessage {
    fn from(value: &EmbeddedResource) -> Self {
        <PromptMessage as ::std::convert::From<PromptMessageContent>>::from(
            <PromptMessageContent as ::std::convert::From<&EmbeddedResource>>::from(value),
        )
    }
}
impl ::std::convert::From<&EmbeddedResource> for PromptMessageContent {
    fn from(value: &EmbeddedResource) -> Self {
        <PromptMessageContent as ::std::convert::From<EmbeddedResource>>::from(
            <EmbeddedResource as ::std::convert::From<&EmbeddedResource>>::from(value),
        )
    }
}
impl ::std::convert::From<&EmbeddedResource> for CallToolResult {
    fn from(value: &EmbeddedResource) -> Self {
        <CallToolResult as ::std::convert::From<CallToolResultContentItem>>::from(
            <CallToolResultContentItem as ::std::convert::From<&EmbeddedResource>>::from(value),
        )
    }
}
impl ::std::convert::From<&EmbeddedResource> for ServerResult {
    fn from(value: &EmbeddedResource) -> Self {
        <ServerResult as ::std::convert::From<CallToolResult>>::from(
            <CallToolResult as ::std::convert::From<&EmbeddedResource>>::from(value),
        )
    }
}
impl ::std::convert::From<&EmbeddedResource> for GetPromptResult {
    fn from(value: &EmbeddedResource) -> Self {
        <GetPromptResult as ::std::convert::From<PromptMessage>>::from(
            <PromptMessage as ::std::convert::From<&EmbeddedResource>>::from(value),
        )
    }
}
impl ::std::convert::From<&EmbeddedResource> for CallToolResultContentItem {
    fn from(value: &EmbeddedResource) -> Self {
        <CallToolResultContentItem as ::std::convert::From<EmbeddedResource>>::from(
            <EmbeddedResource as ::std::convert::From<&EmbeddedResource>>::from(value),
        )
    }
}
impl ::std::convert::From<&ListResourceTemplatesRequest> for ClientRequest {
    fn from(value: &ListResourceTemplatesRequest) -> Self {
        < ClientRequest as :: std :: convert :: From < ListResourceTemplatesRequest >> :: from (< ListResourceTemplatesRequest as :: std :: convert :: From < & ListResourceTemplatesRequest >> :: from (value))
    }
}
impl ::std::convert::From<Vec<ReadResourceResultContentsItem>> for ServerResult {
    fn from(value: Vec<ReadResourceResultContentsItem>) -> Self {
        <ServerResult as ::std::convert::From<ReadResourceResult>>::from(
            <ReadResourceResult as ::std::convert::From<Vec<ReadResourceResultContentsItem>>>::from(
                value,
            ),
        )
    }
}
impl ::std::convert::From<&ReadResourceRequest> for ClientRequest {
    fn from(value: &ReadResourceRequest) -> Self {
        <ClientRequest as ::std::convert::From<ReadResourceRequest>>::from(
            <ReadResourceRequest as ::std::convert::From<&ReadResourceRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<&ListToolsRequest> for ClientRequest {
    fn from(value: &ListToolsRequest) -> Self {
        <ClientRequest as ::std::convert::From<ListToolsRequest>>::from(
            <ListToolsRequest as ::std::convert::From<&ListToolsRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<serde_json::Value> for SamplingMessageContent {
    fn from(value: serde_json::Value) -> Self {
        <SamplingMessageContent as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<serde_json::Value>>::from(value),
        )
    }
}
impl ::std::convert::From<serde_json::Value> for GetPromptResult {
    fn from(value: serde_json::Value) -> Self {
        <GetPromptResult as ::std::convert::From<PromptMessage>>::from(
            <PromptMessage as ::std::convert::From<serde_json::Value>>::from(value),
        )
    }
}
impl ::std::convert::From<serde_json::Value> for CallToolResult {
    fn from(value: serde_json::Value) -> Self {
        <CallToolResult as ::std::convert::From<CallToolResultContentItem>>::from(
            <CallToolResultContentItem as ::std::convert::From<serde_json::Value>>::from(value),
        )
    }
}
impl ::std::convert::From<serde_json::Value> for ServerResult {
    fn from(value: serde_json::Value) -> Self {
        <ServerResult as ::std::convert::From<CallToolResult>>::from(
            <CallToolResult as ::std::convert::From<serde_json::Value>>::from(value),
        )
    }
}
impl ::std::convert::From<serde_json::Value> for PromptMessageContent {
    fn from(value: serde_json::Value) -> Self {
        <PromptMessageContent as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<serde_json::Value>>::from(value),
        )
    }
}
impl ::std::convert::From<serde_json::Value> for CallToolResultContentItem {
    fn from(value: serde_json::Value) -> Self {
        <CallToolResultContentItem as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<serde_json::Value>>::from(value),
        )
    }
}
impl ::std::convert::From<serde_json::Value> for CreateMessageResultContent {
    fn from(value: serde_json::Value) -> Self {
        <CreateMessageResultContent as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<serde_json::Value>>::from(value),
        )
    }
}
impl ::std::convert::From<serde_json::Value> for PromptMessage {
    fn from(value: serde_json::Value) -> Self {
        <PromptMessage as ::std::convert::From<PromptMessageContent>>::from(
            <PromptMessageContent as ::std::convert::From<serde_json::Value>>::from(value),
        )
    }
}
impl ::std::convert::From<&CallToolRequest> for ClientRequest {
    fn from(value: &CallToolRequest) -> Self {
        <ClientRequest as ::std::convert::From<CallToolRequest>>::from(
            <CallToolRequest as ::std::convert::From<&CallToolRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<&TextContent> for PromptMessageContent {
    fn from(value: &TextContent) -> Self {
        <PromptMessageContent as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<&TextContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&TextContent> for ServerResult {
    fn from(value: &TextContent) -> Self {
        <ServerResult as ::std::convert::From<CallToolResult>>::from(
            <CallToolResult as ::std::convert::From<&TextContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&TextContent> for GetPromptResult {
    fn from(value: &TextContent) -> Self {
        <GetPromptResult as ::std::convert::From<PromptMessage>>::from(
            <PromptMessage as ::std::convert::From<&TextContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&TextContent> for PromptMessage {
    fn from(value: &TextContent) -> Self {
        <PromptMessage as ::std::convert::From<PromptMessageContent>>::from(
            <PromptMessageContent as ::std::convert::From<&TextContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&TextContent> for CreateMessageResultContent {
    fn from(value: &TextContent) -> Self {
        <CreateMessageResultContent as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<&TextContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&TextContent> for CallToolResult {
    fn from(value: &TextContent) -> Self {
        <CallToolResult as ::std::convert::From<CallToolResultContentItem>>::from(
            <CallToolResultContentItem as ::std::convert::From<&TextContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&TextContent> for CallToolResultContentItem {
    fn from(value: &TextContent) -> Self {
        <CallToolResultContentItem as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<&TextContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&TextContent> for SamplingMessageContent {
    fn from(value: &TextContent) -> Self {
        <SamplingMessageContent as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<&TextContent>>::from(value),
        )
    }
}
impl ::std::convert::From<CompleteResultCompletion> for ServerResult {
    fn from(value: CompleteResultCompletion) -> Self {
        <ServerResult as ::std::convert::From<CompleteResult>>::from(
            <CompleteResult as ::std::convert::From<CompleteResultCompletion>>::from(value),
        )
    }
}
impl ::std::convert::From<CallToolResultContentItem> for ServerResult {
    fn from(value: CallToolResultContentItem) -> Self {
        <ServerResult as ::std::convert::From<CallToolResult>>::from(
            <CallToolResult as ::std::convert::From<CallToolResultContentItem>>::from(value),
        )
    }
}
impl ::std::convert::From<&str> for ReadResourceResult {
    fn from(value: &str) -> Self {
        <ReadResourceResult as ::std::convert::From<ReadResourceResultContentsItem>>::from(
            <ReadResourceResultContentsItem as ::std::convert::From<&str>>::from(value),
        )
    }
}
impl ::std::convert::From<&str> for PromptMessageContent {
    fn from(value: &str) -> Self {
        <PromptMessageContent as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<&str>>::from(value),
        )
    }
}
impl ::std::convert::From<&str> for ReadResourceResultContentsItem {
    fn from(value: &str) -> Self {
        <ReadResourceResultContentsItem as ::std::convert::From<TextResourceContents>>::from(
            <TextResourceContents as ::std::convert::From<&str>>::from(value),
        )
    }
}
impl ::std::convert::From<&str> for CallToolResult {
    fn from(value: &str) -> Self {
        <CallToolResult as ::std::convert::From<CallToolResultContentItem>>::from(
            <CallToolResultContentItem as ::std::convert::From<&str>>::from(value),
        )
    }
}
impl ::std::convert::From<&str> for GetPromptResult {
    fn from(value: &str) -> Self {
        <GetPromptResult as ::std::convert::From<PromptMessage>>::from(
            <PromptMessage as ::std::convert::From<&str>>::from(value),
        )
    }
}
impl ::std::convert::From<&str> for SamplingMessageContent {
    fn from(value: &str) -> Self {
        <SamplingMessageContent as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<&str>>::from(value),
        )
    }
}
impl ::std::convert::From<&str> for CreateMessageResultContent {
    fn from(value: &str) -> Self {
        <CreateMessageResultContent as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<&str>>::from(value),
        )
    }
}
impl ::std::convert::From<&str> for EmbeddedResourceResource {
    fn from(value: &str) -> Self {
        <EmbeddedResourceResource as ::std::convert::From<TextResourceContents>>::from(
            <TextResourceContents as ::std::convert::From<&str>>::from(value),
        )
    }
}
impl ::std::convert::From<&str> for CallToolResultContentItem {
    fn from(value: &str) -> Self {
        <CallToolResultContentItem as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<&str>>::from(value),
        )
    }
}
impl ::std::convert::From<&str> for PromptMessage {
    fn from(value: &str) -> Self {
        <PromptMessage as ::std::convert::From<PromptMessageContent>>::from(
            <PromptMessageContent as ::std::convert::From<&str>>::from(value),
        )
    }
}
impl ::std::convert::From<ReadResourceResultContentsItem> for ServerResult {
    fn from(value: ReadResourceResultContentsItem) -> Self {
        <ServerResult as ::std::convert::From<ReadResourceResult>>::from(
            <ReadResourceResult as ::std::convert::From<ReadResourceResultContentsItem>>::from(
                value,
            ),
        )
    }
}
impl ::std::convert::From<BlobResourceContents> for ServerResult {
    fn from(value: BlobResourceContents) -> Self {
        <ServerResult as ::std::convert::From<ReadResourceResult>>::from(
            <ReadResourceResult as ::std::convert::From<BlobResourceContents>>::from(value),
        )
    }
}
impl ::std::convert::From<BlobResourceContents> for ReadResourceResult {
    fn from(value: BlobResourceContents) -> Self {
        <ReadResourceResult as ::std::convert::From<ReadResourceResultContentsItem>>::from(
            <ReadResourceResultContentsItem as ::std::convert::From<BlobResourceContents>>::from(
                value,
            ),
        )
    }
}
impl ::std::convert::From<&LoggingMessageNotification> for ServerNotification {
    fn from(value: &LoggingMessageNotification) -> Self {
        <ServerNotification as ::std::convert::From<LoggingMessageNotification>>::from(
            <LoggingMessageNotification as ::std::convert::From<&LoggingMessageNotification>>::from(
                value,
            ),
        )
    }
}
impl ::std::convert::From<&ImageContent> for PromptMessageContent {
    fn from(value: &ImageContent) -> Self {
        <PromptMessageContent as ::std::convert::From<ImageContent>>::from(
            <ImageContent as ::std::convert::From<&ImageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&ImageContent> for SamplingMessageContent {
    fn from(value: &ImageContent) -> Self {
        <SamplingMessageContent as ::std::convert::From<ImageContent>>::from(
            <ImageContent as ::std::convert::From<&ImageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&ImageContent> for CallToolResult {
    fn from(value: &ImageContent) -> Self {
        <CallToolResult as ::std::convert::From<CallToolResultContentItem>>::from(
            <CallToolResultContentItem as ::std::convert::From<&ImageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&ImageContent> for ServerResult {
    fn from(value: &ImageContent) -> Self {
        <ServerResult as ::std::convert::From<CallToolResult>>::from(
            <CallToolResult as ::std::convert::From<&ImageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&ImageContent> for PromptMessage {
    fn from(value: &ImageContent) -> Self {
        <PromptMessage as ::std::convert::From<PromptMessageContent>>::from(
            <PromptMessageContent as ::std::convert::From<&ImageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&ImageContent> for CreateMessageResultContent {
    fn from(value: &ImageContent) -> Self {
        <CreateMessageResultContent as ::std::convert::From<ImageContent>>::from(
            <ImageContent as ::std::convert::From<&ImageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&ImageContent> for GetPromptResult {
    fn from(value: &ImageContent) -> Self {
        <GetPromptResult as ::std::convert::From<PromptMessage>>::from(
            <PromptMessage as ::std::convert::From<&ImageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&ImageContent> for CallToolResultContentItem {
    fn from(value: &ImageContent) -> Self {
        <CallToolResultContentItem as ::std::convert::From<ImageContent>>::from(
            <ImageContent as ::std::convert::From<&ImageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&PromptMessageContent> for ServerResult {
    fn from(value: &PromptMessageContent) -> Self {
        <ServerResult as ::std::convert::From<GetPromptResult>>::from(
            <GetPromptResult as ::std::convert::From<&PromptMessageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&PromptMessageContent> for PromptMessage {
    fn from(value: &PromptMessageContent) -> Self {
        <PromptMessage as ::std::convert::From<PromptMessageContent>>::from(
            <PromptMessageContent as ::std::convert::From<&PromptMessageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&PromptMessageContent> for GetPromptResult {
    fn from(value: &PromptMessageContent) -> Self {
        <GetPromptResult as ::std::convert::From<PromptMessage>>::from(
            <PromptMessage as ::std::convert::From<&PromptMessageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<PromptMessageContent> for GetPromptResult {
    fn from(value: PromptMessageContent) -> Self {
        <GetPromptResult as ::std::convert::From<PromptMessage>>::from(
            <PromptMessage as ::std::convert::From<PromptMessageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<PromptMessageContent> for ServerResult {
    fn from(value: PromptMessageContent) -> Self {
        <ServerResult as ::std::convert::From<GetPromptResult>>::from(
            <GetPromptResult as ::std::convert::From<PromptMessageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&JsonrpcResponse> for JsonrpcMessage {
    fn from(value: &JsonrpcResponse) -> Self {
        <JsonrpcMessage as ::std::convert::From<JsonrpcResponse>>::from(
            <JsonrpcResponse as ::std::convert::From<&JsonrpcResponse>>::from(value),
        )
    }
}
impl ::std::convert::From<&ListToolsResult> for ServerResult {
    fn from(value: &ListToolsResult) -> Self {
        <ServerResult as ::std::convert::From<ListToolsResult>>::from(
            <ListToolsResult as ::std::convert::From<&ListToolsResult>>::from(value),
        )
    }
}
impl ::std::convert::From<&ListRootsRequest> for ServerRequest {
    fn from(value: &ListRootsRequest) -> Self {
        <ServerRequest as ::std::convert::From<ListRootsRequest>>::from(
            <ListRootsRequest as ::std::convert::From<&ListRootsRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<&SetLevelRequest> for ClientRequest {
    fn from(value: &SetLevelRequest) -> Self {
        <ClientRequest as ::std::convert::From<SetLevelRequest>>::from(
            <SetLevelRequest as ::std::convert::From<&SetLevelRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<&SubscribeRequest> for ClientRequest {
    fn from(value: &SubscribeRequest) -> Self {
        <ClientRequest as ::std::convert::From<SubscribeRequest>>::from(
            <SubscribeRequest as ::std::convert::From<&SubscribeRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<&ListResourceTemplatesResult> for ServerResult {
    fn from(value: &ListResourceTemplatesResult) -> Self {
        < ServerResult as :: std :: convert :: From < ListResourceTemplatesResult >> :: from (< ListResourceTemplatesResult as :: std :: convert :: From < & ListResourceTemplatesResult >> :: from (value))
    }
}
impl ::std::convert::From<&ReadResourceResult> for ServerResult {
    fn from(value: &ReadResourceResult) -> Self {
        <ServerResult as ::std::convert::From<ReadResourceResult>>::from(
            <ReadResourceResult as ::std::convert::From<&ReadResourceResult>>::from(value),
        )
    }
}
impl ::std::convert::From<&InitializeRequest> for ClientRequest {
    fn from(value: &InitializeRequest) -> Self {
        <ClientRequest as ::std::convert::From<InitializeRequest>>::from(
            <InitializeRequest as ::std::convert::From<&InitializeRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<TextContent> for ServerResult {
    fn from(value: TextContent) -> Self {
        <ServerResult as ::std::convert::From<CallToolResult>>::from(
            <CallToolResult as ::std::convert::From<TextContent>>::from(value),
        )
    }
}
impl ::std::convert::From<TextContent> for PromptMessage {
    fn from(value: TextContent) -> Self {
        <PromptMessage as ::std::convert::From<PromptMessageContent>>::from(
            <PromptMessageContent as ::std::convert::From<TextContent>>::from(value),
        )
    }
}
impl ::std::convert::From<TextContent> for GetPromptResult {
    fn from(value: TextContent) -> Self {
        <GetPromptResult as ::std::convert::From<PromptMessage>>::from(
            <PromptMessage as ::std::convert::From<TextContent>>::from(value),
        )
    }
}
impl ::std::convert::From<TextContent> for CallToolResult {
    fn from(value: TextContent) -> Self {
        <CallToolResult as ::std::convert::From<CallToolResultContentItem>>::from(
            <CallToolResultContentItem as ::std::convert::From<TextContent>>::from(value),
        )
    }
}
impl ::std::convert::From<&InitializedNotification> for ClientNotification {
    fn from(value: &InitializedNotification) -> Self {
        <ClientNotification as ::std::convert::From<InitializedNotification>>::from(
            <InitializedNotification as ::std::convert::From<&InitializedNotification>>::from(
                value,
            ),
        )
    }
}
impl ::std::convert::From<Vec<Resource>> for ServerResult {
    fn from(value: Vec<Resource>) -> Self {
        <ServerResult as ::std::convert::From<ListResourcesResult>>::from(
            <ListResourcesResult as ::std::convert::From<Vec<Resource>>>::from(value),
        )
    }
}
impl ::std::convert::From<&ResourceUpdatedNotification> for ServerNotification {
    fn from(value: &ResourceUpdatedNotification) -> Self {
        < ServerNotification as :: std :: convert :: From < ResourceUpdatedNotification >> :: from (< ResourceUpdatedNotification as :: std :: convert :: From < & ResourceUpdatedNotification >> :: from (value))
    }
}
impl ::std::convert::From<&GetPromptRequest> for ClientRequest {
    fn from(value: &GetPromptRequest) -> Self {
        <ClientRequest as ::std::convert::From<GetPromptRequest>>::from(
            <GetPromptRequest as ::std::convert::From<&GetPromptRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<&GetPromptResult> for ServerResult {
    fn from(value: &GetPromptResult) -> Self {
        <ServerResult as ::std::convert::From<GetPromptResult>>::from(
            <GetPromptResult as ::std::convert::From<&GetPromptResult>>::from(value),
        )
    }
}
impl ::std::convert::From<&ResourceReference> for CompleteRequestParamsRef {
    fn from(value: &ResourceReference) -> Self {
        <CompleteRequestParamsRef as ::std::convert::From<ResourceReference>>::from(
            <ResourceReference as ::std::convert::From<&ResourceReference>>::from(value),
        )
    }
}
impl ::std::convert::From<&BlobResourceContents> for ReadResourceResultContentsItem {
    fn from(value: &BlobResourceContents) -> Self {
        <ReadResourceResultContentsItem as ::std::convert::From<BlobResourceContents>>::from(
            <BlobResourceContents as ::std::convert::From<&BlobResourceContents>>::from(value),
        )
    }
}
impl ::std::convert::From<&BlobResourceContents> for ReadResourceResult {
    fn from(value: &BlobResourceContents) -> Self {
        <ReadResourceResult as ::std::convert::From<ReadResourceResultContentsItem>>::from(
            <ReadResourceResultContentsItem as ::std::convert::From<&BlobResourceContents>>::from(
                value,
            ),
        )
    }
}
impl ::std::convert::From<&BlobResourceContents> for EmbeddedResourceResource {
    fn from(value: &BlobResourceContents) -> Self {
        <EmbeddedResourceResource as ::std::convert::From<BlobResourceContents>>::from(
            <BlobResourceContents as ::std::convert::From<&BlobResourceContents>>::from(value),
        )
    }
}
impl ::std::convert::From<&BlobResourceContents> for ServerResult {
    fn from(value: &BlobResourceContents) -> Self {
        <ServerResult as ::std::convert::From<ReadResourceResult>>::from(
            <ReadResourceResult as ::std::convert::From<&BlobResourceContents>>::from(value),
        )
    }
}
impl ::std::convert::From<&InitializeResult> for ServerResult {
    fn from(value: &InitializeResult) -> Self {
        <ServerResult as ::std::convert::From<InitializeResult>>::from(
            <InitializeResult as ::std::convert::From<&InitializeResult>>::from(value),
        )
    }
}
impl ::std::convert::From<&PingRequest> for ClientRequest {
    fn from(value: &PingRequest) -> Self {
        <ClientRequest as ::std::convert::From<PingRequest>>::from(
            <PingRequest as ::std::convert::From<&PingRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<&PingRequest> for ServerRequest {
    fn from(value: &PingRequest) -> Self {
        <ServerRequest as ::std::convert::From<PingRequest>>::from(
            <PingRequest as ::std::convert::From<&PingRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<&ListRootsResult> for ClientResult {
    fn from(value: &ListRootsResult) -> Self {
        <ClientResult as ::std::convert::From<ListRootsResult>>::from(
            <ListRootsResult as ::std::convert::From<&ListRootsResult>>::from(value),
        )
    }
}
impl ::std::convert::From<&RootsListChangedNotification> for ClientNotification {
    fn from(value: &RootsListChangedNotification) -> Self {
        < ClientNotification as :: std :: convert :: From < RootsListChangedNotification >> :: from (< RootsListChangedNotification as :: std :: convert :: From < & RootsListChangedNotification >> :: from (value))
    }
}
impl ::std::convert::From<&ResourceListChangedNotification> for ServerNotification {
    fn from(value: &ResourceListChangedNotification) -> Self {
        <ServerNotification as ::std::convert::From<ResourceListChangedNotification>>::from(
            <ResourceListChangedNotification as ::std::convert::From<
                &ResourceListChangedNotification,
            >>::from(value),
        )
    }
}
impl ::std::convert::From<()> for ServerResult {
    fn from(value: ()) -> Self {
        <ServerResult as ::std::convert::From<CallToolResult>>::from(
            <CallToolResult as ::std::convert::From<()>>::from(value),
        )
    }
}
impl ::std::convert::From<Vec<String>> for ServerResult {
    fn from(value: Vec<String>) -> Self {
        <ServerResult as ::std::convert::From<CompleteResult>>::from(
            <CompleteResult as ::std::convert::From<Vec<String>>>::from(value),
        )
    }
}
impl ::std::convert::From<Vec<String>> for CompleteResult {
    fn from(value: Vec<String>) -> Self {
        <CompleteResult as ::std::convert::From<CompleteResultCompletion>>::from(
            <CompleteResultCompletion as ::std::convert::From<Vec<String>>>::from(value),
        )
    }
}
impl ::std::convert::From<&CompleteResultCompletion> for CompleteResult {
    fn from(value: &CompleteResultCompletion) -> Self {
        <CompleteResult as ::std::convert::From<CompleteResultCompletion>>::from(
            <CompleteResultCompletion as ::std::convert::From<&CompleteResultCompletion>>::from(
                value,
            ),
        )
    }
}
impl ::std::convert::From<&CompleteResultCompletion> for ServerResult {
    fn from(value: &CompleteResultCompletion) -> Self {
        <ServerResult as ::std::convert::From<CompleteResult>>::from(
            <CompleteResult as ::std::convert::From<&CompleteResultCompletion>>::from(value),
        )
    }
}
impl ::std::convert::From<&JsonrpcRequest> for JsonrpcMessage {
    fn from(value: &JsonrpcRequest) -> Self {
        <JsonrpcMessage as ::std::convert::From<JsonrpcRequest>>::from(
            <JsonrpcRequest as ::std::convert::From<&JsonrpcRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<EmbeddedResource> for CallToolResult {
    fn from(value: EmbeddedResource) -> Self {
        <CallToolResult as ::std::convert::From<CallToolResultContentItem>>::from(
            <CallToolResultContentItem as ::std::convert::From<EmbeddedResource>>::from(value),
        )
    }
}
impl ::std::convert::From<EmbeddedResource> for GetPromptResult {
    fn from(value: EmbeddedResource) -> Self {
        <GetPromptResult as ::std::convert::From<PromptMessage>>::from(
            <PromptMessage as ::std::convert::From<EmbeddedResource>>::from(value),
        )
    }
}
impl ::std::convert::From<EmbeddedResource> for ServerResult {
    fn from(value: EmbeddedResource) -> Self {
        <ServerResult as ::std::convert::From<CallToolResult>>::from(
            <CallToolResult as ::std::convert::From<EmbeddedResource>>::from(value),
        )
    }
}
impl ::std::convert::From<EmbeddedResource> for PromptMessage {
    fn from(value: EmbeddedResource) -> Self {
        <PromptMessage as ::std::convert::From<PromptMessageContent>>::from(
            <PromptMessageContent as ::std::convert::From<EmbeddedResource>>::from(value),
        )
    }
}
impl ::std::convert::From<PromptMessage> for ServerResult {
    fn from(value: PromptMessage) -> Self {
        <ServerResult as ::std::convert::From<GetPromptResult>>::from(
            <GetPromptResult as ::std::convert::From<PromptMessage>>::from(value),
        )
    }
}
impl ::std::convert::From<&Result> for EmptyResult {
    fn from(value: &Result) -> Self {
        <EmptyResult as ::std::convert::From<Result>>::from(<Result as ::std::convert::From<
            &Result,
        >>::from(value))
    }
}
impl ::std::convert::From<&Result> for ServerResult {
    fn from(value: &Result) -> Self {
        <ServerResult as ::std::convert::From<Result>>::from(<Result as ::std::convert::From<
            &Result,
        >>::from(value))
    }
}
impl ::std::convert::From<&Result> for ClientResult {
    fn from(value: &Result) -> Self {
        <ClientResult as ::std::convert::From<Result>>::from(<Result as ::std::convert::From<
            &Result,
        >>::from(value))
    }
}
impl ::std::convert::From<Vec<ResourceTemplate>> for ServerResult {
    fn from(value: Vec<ResourceTemplate>) -> Self {
        <ServerResult as ::std::convert::From<ListResourceTemplatesResult>>::from(
            <ListResourceTemplatesResult as ::std::convert::From<Vec<ResourceTemplate>>>::from(
                value,
            ),
        )
    }
}
impl ::std::convert::From<&CancelledNotification> for ClientNotification {
    fn from(value: &CancelledNotification) -> Self {
        <ClientNotification as ::std::convert::From<CancelledNotification>>::from(
            <CancelledNotification as ::std::convert::From<&CancelledNotification>>::from(value),
        )
    }
}
impl ::std::convert::From<&CancelledNotification> for ServerNotification {
    fn from(value: &CancelledNotification) -> Self {
        <ServerNotification as ::std::convert::From<CancelledNotification>>::from(
            <CancelledNotification as ::std::convert::From<&CancelledNotification>>::from(value),
        )
    }
}
impl ::std::convert::From<Vec<Tool>> for ServerResult {
    fn from(value: Vec<Tool>) -> Self {
        <ServerResult as ::std::convert::From<ListToolsResult>>::from(
            <ListToolsResult as ::std::convert::From<Vec<Tool>>>::from(value),
        )
    }
}
impl ::std::convert::From<Vec<Root>> for ClientResult {
    fn from(value: Vec<Root>) -> Self {
        <ClientResult as ::std::convert::From<ListRootsResult>>::from(
            <ListRootsResult as ::std::convert::From<Vec<Root>>>::from(value),
        )
    }
}
impl ::std::convert::From<&PromptMessage> for GetPromptResult {
    fn from(value: &PromptMessage) -> Self {
        <GetPromptResult as ::std::convert::From<PromptMessage>>::from(
            <PromptMessage as ::std::convert::From<&PromptMessage>>::from(value),
        )
    }
}
impl ::std::convert::From<&PromptMessage> for ServerResult {
    fn from(value: &PromptMessage) -> Self {
        <ServerResult as ::std::convert::From<GetPromptResult>>::from(
            <GetPromptResult as ::std::convert::From<&PromptMessage>>::from(value),
        )
    }
}
impl ::std::convert::From<TextResourceContents> for ServerResult {
    fn from(value: TextResourceContents) -> Self {
        <ServerResult as ::std::convert::From<ReadResourceResult>>::from(
            <ReadResourceResult as ::std::convert::From<TextResourceContents>>::from(value),
        )
    }
}
impl ::std::convert::From<TextResourceContents> for ReadResourceResult {
    fn from(value: TextResourceContents) -> Self {
        <ReadResourceResult as ::std::convert::From<ReadResourceResultContentsItem>>::from(
            <ReadResourceResultContentsItem as ::std::convert::From<TextResourceContents>>::from(
                value,
            ),
        )
    }
}
impl ::std::convert::From<&ReadResourceResultContentsItem> for ReadResourceResult {
    fn from(value: &ReadResourceResultContentsItem) -> Self {
        <ReadResourceResult as ::std::convert::From<ReadResourceResultContentsItem>>::from(
            <ReadResourceResultContentsItem as ::std::convert::From<
                &ReadResourceResultContentsItem,
            >>::from(value),
        )
    }
}
impl ::std::convert::From<&ReadResourceResultContentsItem> for ServerResult {
    fn from(value: &ReadResourceResultContentsItem) -> Self {
        <ServerResult as ::std::convert::From<ReadResourceResult>>::from(
            <ReadResourceResult as ::std::convert::From<&ReadResourceResultContentsItem>>::from(
                value,
            ),
        )
    }
}
impl ::std::convert::From<&PromptListChangedNotification> for ServerNotification {
    fn from(value: &PromptListChangedNotification) -> Self {
        <ServerNotification as ::std::convert::From<PromptListChangedNotification>>::from(
            <PromptListChangedNotification as ::std::convert::From<
                &PromptListChangedNotification,
            >>::from(value),
        )
    }
}
impl ::std::convert::From<&ProgressNotification> for ServerNotification {
    fn from(value: &ProgressNotification) -> Self {
        <ServerNotification as ::std::convert::From<ProgressNotification>>::from(
            <ProgressNotification as ::std::convert::From<&ProgressNotification>>::from(value),
        )
    }
}
impl ::std::convert::From<&ProgressNotification> for ClientNotification {
    fn from(value: &ProgressNotification) -> Self {
        <ClientNotification as ::std::convert::From<ProgressNotification>>::from(
            <ProgressNotification as ::std::convert::From<&ProgressNotification>>::from(value),
        )
    }
}
impl ::std::convert::From<&CompleteRequest> for ClientRequest {
    fn from(value: &CompleteRequest) -> Self {
        <ClientRequest as ::std::convert::From<CompleteRequest>>::from(
            <CompleteRequest as ::std::convert::From<&CompleteRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<&ListResourcesResult> for ServerResult {
    fn from(value: &ListResourcesResult) -> Self {
        <ServerResult as ::std::convert::From<ListResourcesResult>>::from(
            <ListResourcesResult as ::std::convert::From<&ListResourcesResult>>::from(value),
        )
    }
}
impl ::std::convert::From<ImageContent> for GetPromptResult {
    fn from(value: ImageContent) -> Self {
        <GetPromptResult as ::std::convert::From<PromptMessage>>::from(
            <PromptMessage as ::std::convert::From<ImageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<ImageContent> for ServerResult {
    fn from(value: ImageContent) -> Self {
        <ServerResult as ::std::convert::From<CallToolResult>>::from(
            <CallToolResult as ::std::convert::From<ImageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<ImageContent> for CallToolResult {
    fn from(value: ImageContent) -> Self {
        <CallToolResult as ::std::convert::From<CallToolResultContentItem>>::from(
            <CallToolResultContentItem as ::std::convert::From<ImageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<ImageContent> for PromptMessage {
    fn from(value: ImageContent) -> Self {
        <PromptMessage as ::std::convert::From<PromptMessageContent>>::from(
            <PromptMessageContent as ::std::convert::From<ImageContent>>::from(value),
        )
    }
}
impl ::std::convert::From<Vec<Prompt>> for ServerResult {
    fn from(value: Vec<Prompt>) -> Self {
        <ServerResult as ::std::convert::From<ListPromptsResult>>::from(
            <ListPromptsResult as ::std::convert::From<Vec<Prompt>>>::from(value),
        )
    }
}
impl ::std::convert::From<Base64Bytes> for ServerResult {
    fn from(value: Base64Bytes) -> Self {
        <ServerResult as ::std::convert::From<ReadResourceResult>>::from(
            <ReadResourceResult as ::std::convert::From<Base64Bytes>>::from(value),
        )
    }
}
impl ::std::convert::From<Base64Bytes> for EmbeddedResourceResource {
    fn from(value: Base64Bytes) -> Self {
        <EmbeddedResourceResource as ::std::convert::From<BlobResourceContents>>::from(
            <BlobResourceContents as ::std::convert::From<Base64Bytes>>::from(value),
        )
    }
}
impl ::std::convert::From<Base64Bytes> for ReadResourceResult {
    fn from(value: Base64Bytes) -> Self {
        <ReadResourceResult as ::std::convert::From<ReadResourceResultContentsItem>>::from(
            <ReadResourceResultContentsItem as ::std::convert::From<Base64Bytes>>::from(value),
        )
    }
}
impl ::std::convert::From<Base64Bytes> for ReadResourceResultContentsItem {
    fn from(value: Base64Bytes) -> Self {
        <ReadResourceResultContentsItem as ::std::convert::From<BlobResourceContents>>::from(
            <BlobResourceContents as ::std::convert::From<Base64Bytes>>::from(value),
        )
    }
}
impl ::std::convert::From<String> for CallToolResult {
    fn from(value: String) -> Self {
        <CallToolResult as ::std::convert::From<CallToolResultContentItem>>::from(
            <CallToolResultContentItem as ::std::convert::From<String>>::from(value),
        )
    }
}
impl ::std::convert::From<String> for ReadResourceResultContentsItem {
    fn from(value: String) -> Self {
        <ReadResourceResultContentsItem as ::std::convert::From<TextResourceContents>>::from(
            <TextResourceContents as ::std::convert::From<String>>::from(value),
        )
    }
}
impl ::std::convert::From<String> for GetPromptResult {
    fn from(value: String) -> Self {
        <GetPromptResult as ::std::convert::From<PromptMessage>>::from(
            <PromptMessage as ::std::convert::From<String>>::from(value),
        )
    }
}
impl ::std::convert::From<String> for PromptMessage {
    fn from(value: String) -> Self {
        <PromptMessage as ::std::convert::From<PromptMessageContent>>::from(
            <PromptMessageContent as ::std::convert::From<String>>::from(value),
        )
    }
}
impl ::std::convert::From<String> for ReadResourceResult {
    fn from(value: String) -> Self {
        <ReadResourceResult as ::std::convert::From<ReadResourceResultContentsItem>>::from(
            <ReadResourceResultContentsItem as ::std::convert::From<String>>::from(value),
        )
    }
}
impl ::std::convert::From<String> for CallToolResultContentItem {
    fn from(value: String) -> Self {
        <CallToolResultContentItem as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<String>>::from(value),
        )
    }
}
impl ::std::convert::From<String> for SamplingMessageContent {
    fn from(value: String) -> Self {
        <SamplingMessageContent as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<String>>::from(value),
        )
    }
}
impl ::std::convert::From<String> for PromptMessageContent {
    fn from(value: String) -> Self {
        <PromptMessageContent as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<String>>::from(value),
        )
    }
}
impl ::std::convert::From<String> for EmbeddedResourceResource {
    fn from(value: String) -> Self {
        <EmbeddedResourceResource as ::std::convert::From<TextResourceContents>>::from(
            <TextResourceContents as ::std::convert::From<String>>::from(value),
        )
    }
}
impl ::std::convert::From<String> for CreateMessageResultContent {
    fn from(value: String) -> Self {
        <CreateMessageResultContent as ::std::convert::From<TextContent>>::from(
            <TextContent as ::std::convert::From<String>>::from(value),
        )
    }
}
impl ::std::convert::From<&[&str]> for CompleteResult {
    fn from(value: &[&str]) -> Self {
        <CompleteResult as ::std::convert::From<CompleteResultCompletion>>::from(
            <CompleteResultCompletion as ::std::convert::From<&[&str]>>::from(value),
        )
    }
}
impl ::std::convert::From<&[&str]> for ServerResult {
    fn from(value: &[&str]) -> Self {
        <ServerResult as ::std::convert::From<CompleteResult>>::from(
            <CompleteResult as ::std::convert::From<&[&str]>>::from(value),
        )
    }
}
impl ::std::convert::From<&CallToolResult> for ServerResult {
    fn from(value: &CallToolResult) -> Self {
        <ServerResult as ::std::convert::From<CallToolResult>>::from(
            <CallToolResult as ::std::convert::From<&CallToolResult>>::from(value),
        )
    }
}
impl ::std::convert::From<&CallToolResultContentItem> for CallToolResult {
    fn from(value: &CallToolResultContentItem) -> Self {
        <CallToolResult as ::std::convert::From<CallToolResultContentItem>>::from(
            <CallToolResultContentItem as ::std::convert::From<&CallToolResultContentItem>>::from(
                value,
            ),
        )
    }
}
impl ::std::convert::From<&CallToolResultContentItem> for ServerResult {
    fn from(value: &CallToolResultContentItem) -> Self {
        <ServerResult as ::std::convert::From<CallToolResult>>::from(
            <CallToolResult as ::std::convert::From<&CallToolResultContentItem>>::from(value),
        )
    }
}
impl ::std::convert::From<&CompleteResult> for ServerResult {
    fn from(value: &CompleteResult) -> Self {
        <ServerResult as ::std::convert::From<CompleteResult>>::from(
            <CompleteResult as ::std::convert::From<&CompleteResult>>::from(value),
        )
    }
}
impl ::std::convert::From<&EmptyResult> for ServerResult {
    fn from(value: &EmptyResult) -> Self {
        <ServerResult as ::std::convert::From<Result>>::from(<Result as ::std::convert::From<
            &EmptyResult,
        >>::from(value))
    }
}
impl ::std::convert::From<&EmptyResult> for ClientResult {
    fn from(value: &EmptyResult) -> Self {
        <ClientResult as ::std::convert::From<Result>>::from(<Result as ::std::convert::From<
            &EmptyResult,
        >>::from(value))
    }
}
impl ::std::convert::From<&EmptyResult> for Result {
    fn from(value: &EmptyResult) -> Self {
        <Result as ::std::convert::From<EmptyResult>>::from(<EmptyResult as ::std::convert::From<
            &EmptyResult,
        >>::from(value))
    }
}
impl ::std::convert::From<&UnsubscribeRequest> for ClientRequest {
    fn from(value: &UnsubscribeRequest) -> Self {
        <ClientRequest as ::std::convert::From<UnsubscribeRequest>>::from(
            <UnsubscribeRequest as ::std::convert::From<&UnsubscribeRequest>>::from(value),
        )
    }
}
impl ::std::convert::From<&ListResourcesRequest> for ClientRequest {
    fn from(value: &ListResourcesRequest) -> Self {
        <ClientRequest as ::std::convert::From<ListResourcesRequest>>::from(
            <ListResourcesRequest as ::std::convert::From<&ListResourcesRequest>>::from(value),
        )
    }
}
