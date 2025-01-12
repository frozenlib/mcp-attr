use mcp_attr::server::{mcp_server, McpServer};

struct MyMcpServer;

#[mcp_server]
impl McpServer for MyMcpServer {}

#[test]
fn test() {
    let server = MyMcpServer;
    fn f(_: impl McpServer) {}
    f(server);
}
