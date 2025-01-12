use mcp_attr::{
    server::{mcp_server, McpServer},
    Result,
};
struct InvalidArg;

struct MyServer;

#[mcp_server]
impl McpServer for MyServer {
    #[tool]
    async fn f(&self, a: &str) -> Result<String> {
        todo!()
    }
}

fn main() {}
