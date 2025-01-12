use mcp_attr::server::{mcp_server, McpServer};
use mcp_attr::Result;

struct MyMcpServer;

#[mcp_server]
impl McpServer for MyMcpServer {
    #[tool("custom_name")]
    async fn my_tool_arg(&self, _arg: String) -> Result<()> {
        Ok(())
    }
}

#[test]
fn test() {
    let server = MyMcpServer;
    fn f(_: impl McpServer) {}
    f(server);
}
