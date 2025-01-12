use mcp_attr::server::{mcp_server, McpServer};
use mcp_attr::Result;

struct MyMcpServer;

#[mcp_server]
impl McpServer for MyMcpServer {
    #[prompt]
    async fn my_prompt(&self) -> Result<&str> {
        Ok("Hello, world!")
    }
}

#[test]
fn test() {
    let server = MyMcpServer;
    fn f(_: impl McpServer) {}
    f(server);
}
