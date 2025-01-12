use std::{future::Future, sync::Arc};

use derive_ex::Ex;
use jsoncall::{
    Handler, NotificationContext, Params, RequestContext, RequestContextAs, Response, Result,
    Session, SessionResult,
};
use serde_json::Map;
use tokio::io::{AsyncBufRead, AsyncWrite};

use crate::{
    common::McpCancellationHook,
    schema::{
        CallToolRequestParams, CallToolResult, CancelledNotificationParams, ClientCapabilities,
        ClientCapabilitiesRoots, CreateMessageRequestParams, CreateMessageResult,
        GetPromptRequestParams, GetPromptResult, Implementation, InitializeRequestParams,
        InitializeResult, InitializedNotificationParams, ListPromptsRequestParams,
        ListPromptsResult, ListResourceTemplatesRequestParams, ListResourceTemplatesResult,
        ListResourcesRequestParams, ListResourcesResult, ListToolsRequestParams, ListToolsResult,
        PingRequestParams, ReadResourceRequestParams, ReadResourceResult, Root,
    },
    server::McpServer,
    utils::Empty,
    PROTOCOL_VERSION,
};

pub struct McpclientSessionOptions {}

pub trait SamplingHandler {
    fn create_message(
        &self,
        p: CreateMessageRequestParams,
    ) -> impl Future<Output = Result<CreateMessageResult>> + Send;
}
trait DynSamplingHandler: Send + Sync + 'static {
    fn dyn_create_message(
        self: Arc<Self>,
        p: CreateMessageRequestParams,
        cx: RequestContextAs<CreateMessageResult>,
    ) -> Result<Response>;
}
impl<T: SamplingHandler + Send + Sync + 'static> DynSamplingHandler for T {
    fn dyn_create_message(
        self: Arc<Self>,
        p: CreateMessageRequestParams,
        cx: RequestContextAs<CreateMessageResult>,
    ) -> Result<Response> {
        cx.handle_async(async move { self.create_message(p).await })
    }
}
#[derive(Ex)]
#[derive_ex(Default)]
#[default(Self::new())]
pub struct McpClientBuilder {
    sampling_handler: Option<Arc<dyn DynSamplingHandler>>,
    roots: Option<Vec<Root>>,
    client_info: Implementation,
}
impl McpClientBuilder {
    pub fn new() -> Self {
        Self {
            sampling_handler: None,
            roots: None,
            client_info: Implementation::from_compile_time_env(),
        }
    }
    pub fn with_sampling_handler(
        mut self,
        sampling_handler: impl SamplingHandler + Send + Sync + 'static,
    ) -> Self {
        self.sampling_handler = Some(Arc::new(sampling_handler));
        self
    }
    pub fn with_roots(mut self, roots: Vec<Root>) -> Self {
        self.roots = Some(roots);
        self
    }
    pub fn into_handler(self) -> (impl Handler, InitializeRequestParams) {
        let mut capabilities = ClientCapabilities::default();
        if self.roots.is_some() {
            capabilities.roots = Some(ClientCapabilitiesRoots {
                list_changed: Some(true),
            });
        }
        if self.sampling_handler.is_some() {
            capabilities.sampling = Some(Map::new());
        }
        let handler = MpcClientHandler {
            sampling_handler: self.sampling_handler,
            roots: self.roots,
        };
        let p = InitializeRequestParams {
            capabilities,
            client_info: self.client_info,
            protocol_version: PROTOCOL_VERSION.to_string(),
        };
        (handler, p)
    }

    pub async fn build(
        self,
        reader: impl AsyncBufRead + Send + Sync + 'static,
        writer: impl AsyncWrite + Send + Sync + 'static,
    ) -> SessionResult<McpClient> {
        let (handler, p) = self.into_handler();
        McpClient::initialize(Session::new(handler, reader, writer), p).await
    }
    pub async fn build_with_server(self, server: impl McpServer) -> SessionResult<McpClient> {
        let (client_handler, p) = self.into_handler();
        let server_handler = server.into_handler();

        let (client, server) = Session::new_channel(client_handler, server_handler);
        let mut client = McpClient::initialize(client, p).await?;
        client.server = Some(server);
        Ok(client)
    }
}

struct MpcClientHandler {
    sampling_handler: Option<Arc<dyn DynSamplingHandler>>,
    roots: Option<Vec<Root>>,
}
impl Handler for MpcClientHandler {
    fn hook(&self) -> Arc<dyn jsoncall::Hook> {
        Arc::new(McpCancellationHook)
    }
    fn request(&mut self, method: &str, params: Params, cx: RequestContext) -> Result<Response> {
        match method {
            "sampling/createMessage" => {
                if let Some(h) = &self.sampling_handler {
                    return h.clone().dyn_create_message(params.to()?, cx.to());
                }
            }
            "ping" => return cx.handle(self.ping(params.to()?)),
            "roots/list" => {
                if let Some(roots) = &self.roots {
                    return cx.handle(Ok(roots.clone()));
                }
            }
            _ => {}
        }
        cx.method_not_found()
    }

    fn notification(
        &mut self,
        method: &str,
        params: Params,
        cx: NotificationContext,
    ) -> Result<Response> {
        match method {
            "notifications/cancelled" => self.notifications_cancelled(params.to()?, cx),
            _ => cx.method_not_found(),
        }
    }
}
impl MpcClientHandler {
    fn ping(&self, _p: PingRequestParams) -> Result<Empty> {
        Ok(Empty::default())
    }
    fn notifications_cancelled(
        &self,
        p: CancelledNotificationParams,
        cx: NotificationContext,
    ) -> Result<Response> {
        cx.session().cancel_incoming_request(&p.request_id, None);
        cx.handle(Ok(()))
    }
}
pub struct McpClient {
    session: Session,
    init: InitializeResult,
    server: Option<Session>,
}

impl McpClient {
    pub async fn from_server(server: impl McpServer) -> SessionResult<Self> {
        McpClientBuilder::new().build_with_server(server).await
    }

    pub async fn initialize(session: Session, p: InitializeRequestParams) -> SessionResult<Self> {
        let init = session
            .request::<InitializeResult>("initialize", Some(&p))
            .await?;
        session.notification(
            "notifications/initialized",
            Some(&InitializedNotificationParams::default()),
        )?;
        Ok(Self {
            session,
            init,
            server: None,
        })
    }
    pub fn instructions(&self) -> Option<&str> {
        self.init.instructions.as_deref()
    }
    pub fn server_info(&self) -> &Implementation {
        &self.init.server_info
    }

    pub async fn prompts_list(
        &self,
        params: Option<ListPromptsRequestParams>,
    ) -> SessionResult<ListPromptsResult> {
        self.session.request("prompts/list", params.as_ref()).await
    }
    pub async fn prompts_get(
        &self,
        params: GetPromptRequestParams,
    ) -> SessionResult<GetPromptResult> {
        self.session.request("prompts/get", Some(&params)).await
    }

    pub async fn resources_list(
        &self,
        params: Option<ListResourcesRequestParams>,
    ) -> SessionResult<ListResourcesResult> {
        self.session
            .request("resources/list", params.as_ref())
            .await
    }

    pub async fn resources_templates_list(
        &self,
        params: Option<ListResourceTemplatesRequestParams>,
    ) -> SessionResult<ListResourceTemplatesResult> {
        self.session
            .request("resources/templates/list", params.as_ref())
            .await
    }

    pub async fn resources_read(
        &self,
        params: ReadResourceRequestParams,
    ) -> SessionResult<ReadResourceResult> {
        self.session.request("resources/read", Some(&params)).await
    }

    pub async fn tools_list(
        &self,
        params: Option<ListToolsRequestParams>,
    ) -> SessionResult<ListToolsResult> {
        self.session.request("tools/list", params.as_ref()).await
    }

    pub async fn tools_call(&self, params: CallToolRequestParams) -> SessionResult<CallToolResult> {
        self.session.request("tools/call", Some(&params)).await
    }

    pub async fn ping(&self) -> SessionResult<()> {
        let _: Empty = self
            .session
            .request("ping", Some(&PingRequestParams::default()))
            .await?;
        Ok(())
    }
}
