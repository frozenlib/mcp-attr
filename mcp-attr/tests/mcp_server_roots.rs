use std::env::current_dir;

use pretty_assertions::assert_eq;
use tokio::test;

use mcp_attr::Result;
use mcp_attr::client::McpClientBuilder;
use mcp_attr::schema::{CallToolRequestParams, CallToolResult, CallToolResultContentItem, Root};
use mcp_attr::server::{McpServer, RequestContext, mcp_server};

struct MyMcpServer;

#[mcp_server]
impl McpServer for MyMcpServer {
    #[tool]
    async fn echo_roots(&self, cx: &RequestContext) -> Result<Vec<String>> {
        eprintln!("echo_roots");
        let roots = cx.list_roots().await?;
        eprintln!("list_roots_finished");
        let mut res = Vec::new();
        for root in roots {
            if let Some(path) = root.to_file_path() {
                res.push(path.display().to_string());
            } else {
                res.push("no_roots".to_string());
            }
        }
        Ok(res)
    }
}

#[test]
async fn list_roots() -> Result<()> {
    let files = [current_dir().unwrap().display().to_string()];

    let roots = files
        .iter()
        .map(|f| Root::from_file_path(f).unwrap())
        .collect();

    let client = McpClientBuilder::new()
        .with_roots(roots)
        .build_with_server(MyMcpServer)
        .await?;
    let a = client
        .tools_call(CallToolRequestParams::new("echo_roots"))
        .await?;
    let e: Vec<CallToolResultContentItem> = files.iter().map(|x| x.as_str().into()).collect();
    let e: CallToolResult = e.into();
    assert_eq!(a, e);
    Ok(())
}
