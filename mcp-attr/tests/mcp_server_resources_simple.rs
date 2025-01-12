use mcp_attr::schema::TextResourceContents;
use mcp_attr::server::{mcp_server, McpServer};
use mcp_attr::Result;

struct MyMcpServer;

#[mcp_server]
impl McpServer for MyMcpServer {
    #[resource("http://{a}/{b}")]
    async fn my_resource(&self, a: String, b: String) -> Result<TextResourceContents> {
        let text = format!("Hello, world! {a} {b}");
        Ok(TextResourceContents {
            mime_type: None,
            text,
            uri: format!("http://{a}/{b}"),
        })
    }
}

#[test]
fn test() {
    let server = MyMcpServer;
    fn f(_: impl McpServer) {}
    f(server);
}
