use mcp_attr::{
    schema::{ListResourcesRequestParams, ListResourcesResult, Resource},
    server::{mcp_server, McpServer, RequestContext},
    Result,
};

struct MyMcpServer;

#[mcp_server]
impl McpServer for MyMcpServer {
    async fn resources_list(
        &self,
        _p: ListResourcesRequestParams,
        _cx: &mut RequestContext,
    ) -> Result<ListResourcesResult> {
        Ok(vec![Resource::new("http://a", "a")].into())
    }
}

#[test]
fn test() {
    let server = MyMcpServer;
    fn f(_: impl McpServer) {}
    f(server);
}
