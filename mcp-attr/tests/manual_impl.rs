use jsoncall::Result;
use mcp_attr::{
    schema::{ListPromptsRequestParams, ListPromptsResult, Prompt},
    server::{McpServer, RequestContext},
};
use tokio::time::sleep;

pub struct MyMcpServer {
    prompts: Vec<Prompt>,
}

impl McpServer for MyMcpServer {
    async fn prompts_list(
        &self,
        _p: ListPromptsRequestParams,
        cx: &mut RequestContext,
    ) -> Result<ListPromptsResult> {
        sleep(std::time::Duration::from_secs(1)).await;
        let _cx = cx;
        Ok(self.prompts.clone().into())
    }
}
