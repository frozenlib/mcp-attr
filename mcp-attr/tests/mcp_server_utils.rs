use tokio::test;

use mcp_attr::{
    client::McpClient,
    server::{mcp_server, McpServer},
    Result,
};

struct MyMcpServer;

#[mcp_server]
impl McpServer for MyMcpServer {}

#[test]
async fn ping_client_to_server() -> Result<()> {
    let server = MyMcpServer;
    let client = McpClient::from_server(server).await?;
    client.ping().await?;
    Ok(())
}
