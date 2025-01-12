use jsoncall::{ErrorCode, SessionResult};
use pretty_assertions::assert_eq;
use tokio::test;

use mcp_attr::client::McpClient;
use mcp_attr::schema::{
    GetPromptRequestParams, GetPromptResult, ListPromptsRequestParams, ListPromptsResult, Prompt,
    PromptArgument,
};
use mcp_attr::server::{mcp_server, McpServer};
use mcp_attr::Result;

struct MyMcpServer;

#[mcp_server]
impl McpServer for MyMcpServer {
    #[prompt]
    async fn no_arg(&self) -> Result<&str> {
        Ok("value_no_arg")
    }

    #[prompt("xxx")]
    async fn with_name(&self) -> Result<&str> {
        Ok("value_with_name")
    }

    #[prompt]
    async fn arg_1(&self, arg_1: String) -> Result<String> {
        Ok(format!("hello {arg_1}"))
    }

    #[prompt]
    async fn arg_2(&self, arg_1: String, arg_2: String) -> Result<String> {
        Ok(format!("{arg_1} {arg_2}"))
    }

    #[prompt]
    async fn arg_opt(&self, arg_1: Option<String>) -> Result<String> {
        if let Some(arg_1) = arg_1 {
            Ok(format!("hello {arg_1}"))
        } else {
            Ok("---".to_string())
        }
    }

    #[prompt]
    async fn arg_name_underscore(&self, _arg: String) -> Result<String> {
        Ok(format!("hello {_arg}"))
    }
    #[prompt]
    async fn arg_name_underscore_2(&self, __arg: String) -> Result<String> {
        Ok(format!("hello {__arg}"))
    }

    #[prompt]
    async fn arg_attr_name(&self, #[arg("xxx")] arg: String) -> Result<String> {
        Ok(format!("hello {arg}"))
    }

    #[prompt]
    async fn arg_attr_name_underscore(&self, #[arg("_xxx")] arg: String) -> Result<String> {
        Ok(format!("hello {arg}"))
    }

    /// Prompt Description
    #[prompt]
    async fn prompt_description(&self) -> Result<&str> {
        Ok("prompt_description")
    }

    #[prompt]
    async fn arg_description(
        &self,
        /// Arg description
        arg: String,
    ) -> Result<String> {
        Ok(format!("hello {arg}"))
    }
}

#[test]
async fn list_some() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .prompts_list(Some(ListPromptsRequestParams::default()))
        .await?;
    let e = prompts_expected();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn list_none() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client.prompts_list(None).await?;
    let e = prompts_expected();
    assert_eq!(a, e);
    Ok(())
}

fn prompts_expected() -> ListPromptsResult {
    vec![
        Prompt::new("no_arg"),
        Prompt::new("xxx"),
        Prompt::new("arg_1").with_arguments(vec![PromptArgument::new("arg_1", true)]),
        Prompt::new("arg_2").with_arguments(vec![
            PromptArgument::new("arg_1", true),
            PromptArgument::new("arg_2", true),
        ]),
        Prompt::new("arg_opt").with_arguments(vec![PromptArgument::new("arg_1", false)]),
        Prompt::new("arg_name_underscore").with_arguments(vec![PromptArgument::new("arg", true)]),
        Prompt::new("arg_name_underscore_2")
            .with_arguments(vec![PromptArgument::new("_arg", true)]),
        Prompt::new("arg_attr_name").with_arguments(vec![PromptArgument::new("xxx", true)]),
        Prompt::new("arg_attr_name_underscore")
            .with_arguments(vec![PromptArgument::new("_xxx", true)]),
        Prompt::new("prompt_description").with_description("Prompt Description"),
        Prompt::new("arg_description").with_arguments(vec![
            PromptArgument::new("arg", true).with_description("Arg description")
        ]),
    ]
    .into()
}

#[test]
async fn get_no_arg() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .prompts_get(GetPromptRequestParams::new("no_arg"))
        .await?;
    let e: GetPromptResult = "value_no_arg".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn get_with_name() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .prompts_get(GetPromptRequestParams::new("xxx"))
        .await?;
    let e: GetPromptResult = "value_with_name".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn get_arg_1() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .prompts_get(GetPromptRequestParams::new("arg_1").with_arguments(vec![("arg_1", "world")]))
        .await?;
    let e: GetPromptResult = "hello world".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn get_arg_2() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .prompts_get(
            GetPromptRequestParams::new("arg_2")
                .with_arguments(vec![("arg_1", "aaa"), ("arg_2", "bbb")]),
        )
        .await?;
    let e: GetPromptResult = "aaa bbb".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn get_arg_opt_some() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .prompts_get(GetPromptRequestParams::new("arg_opt").with_arguments(vec![("arg_1", "aaa")]))
        .await?;
    let e: GetPromptResult = "hello aaa".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn get_arg_opt_none() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .prompts_get(GetPromptRequestParams::new("arg_opt"))
        .await?;
    let e: GetPromptResult = "---".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn get_missing_arg() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .prompts_get(GetPromptRequestParams::new("arg_1"))
        .await;
    assert_error(a, ErrorCode::INVALID_PARAMS);
    Ok(())
}

#[test]
async fn get_name_mismatch() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .prompts_get(GetPromptRequestParams::new("unknown"))
        .await;
    assert_error(a, ErrorCode::METHOD_NOT_FOUND);
    Ok(())
}

fn assert_error<T: std::fmt::Debug>(a: SessionResult<T>, code: ErrorCode) {
    match a {
        Ok(_) => panic!("expected error.\n{a:#?}"),
        Err(e) => {
            if let Some(e) = e.error_object() {
                assert_eq!(e.code, code, "{e:#?}");
            } else {
                panic!("no error object\n{e:#?}");
            }
        }
    }
}
