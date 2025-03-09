use tokio::test;

use mcp_attr::{
    Result,
    client::McpClient,
    schema::{
        CompleteRequestParams, CompleteRequestParamsArgument, CompleteRequestParamsRef,
        CompleteResult,
    },
    server::{McpServer, RequestContext},
};

struct MyMcpServer;

impl McpServer for MyMcpServer {
    async fn completion_complete(
        &self,
        _p: CompleteRequestParams,
        _cx: &mut RequestContext,
    ) -> Result<CompleteResult> {
        Ok(CompleteResult::from(["a", "b", "c"].as_slice()))
    }
}

#[test]
async fn completion_complete() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let e = client
        .completion_complete(CompleteRequestParams::new(
            CompleteRequestParamsRef::new_prompt("a"),
            CompleteRequestParamsArgument::new("x", "y"),
        ))
        .await?;
    let a = CompleteResult::from(["a", "b", "c"].as_slice());
    assert_eq!(e, a);
    Ok(())
}
