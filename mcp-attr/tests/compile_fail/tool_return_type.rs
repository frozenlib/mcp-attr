use mcp_attr::{
    server::{mcp_server, McpServer},
    Result,
};
struct X;

struct MyServer;

#[mcp_server]
impl McpServer for MyServer {
    #[tool]
    async fn f(&self) -> Result<X> {
        todo!()
    }
}

fn main() {}
