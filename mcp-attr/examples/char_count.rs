#![windows_subsystem = "windows"]

use mcp_attr::{
    server::{mcp_server, serve_stdio, McpServer},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    serve_stdio(MyServer).await?;
    Ok(())
}

struct MyServer;

#[mcp_server]
impl McpServer for MyServer {
    /// 文字列をカウントするツール
    #[tool]
    async fn char_count(
        &self,
        /// カウント対象の文字列
        text: Vec<String>,
    ) -> Result<Vec<String>> {
        Ok(text.iter().map(|s| s.chars().count().to_string()).collect())
    }
}
