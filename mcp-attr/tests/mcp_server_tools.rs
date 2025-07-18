use jsoncall::{ErrorCode, SessionResult};
use mcp_attr::Result;
use mcp_attr::client::McpClient;
use mcp_attr::schema::{
    CallToolRequestParams, CallToolResult, ListToolsRequestParams, ListToolsResult, Tool,
    ToolInputSchema,
};
use mcp_attr::server::{McpServer, mcp_server};
use pretty_assertions::assert_eq;
use serde_json::Value;
use tokio::test;

struct MyMcpServer;

#[mcp_server]
impl McpServer for MyMcpServer {
    #[tool]
    async fn no_arg(&self) -> Result<&str> {
        Ok("abc")
    }

    #[tool("xxx")]
    async fn with_name(&self) -> Result<&str> {
        Ok("def")
    }

    #[tool]
    async fn arg_1(&self, arg_1: String) -> Result<String> {
        Ok(arg_1)
    }

    #[tool]
    async fn arg_2(&self, arg_1: String, arg_2: String) -> Result<String> {
        Ok(format!("{arg_1} {arg_2}"))
    }

    #[tool]
    async fn arg_opt(&self, arg_1: Option<String>) -> Result<String> {
        if let Some(arg_1) = arg_1 {
            Ok(arg_1)
        } else {
            Ok("---".to_string())
        }
    }

    #[tool]
    async fn arg_name_underscore(&self, _arg: String) -> Result<String> {
        Ok(_arg)
    }

    #[tool]
    async fn arg_name_underscore_2(&self, __arg: String) -> Result<String> {
        Ok(__arg)
    }

    #[tool]
    async fn arg_attr_name(&self, #[arg("xxx")] arg: String) -> Result<String> {
        Ok(arg)
    }

    #[tool]
    async fn arg_attr_name_underscore(&self, #[arg("_xxx")] arg: String) -> Result<String> {
        Ok(arg)
    }

    /// Tool description
    #[tool]
    async fn tool_description(&self) -> Result<()> {
        Ok(())
    }

    /// Tool description line1
    /// Tool description line2
    #[tool]
    async fn tool_description_multiline(&self) -> Result<()> {
        Ok(())
    }

    #[tool]
    async fn arg_description(
        &self,
        /// Arg description
        arg: String,
    ) -> Result<String> {
        Ok(format!("hello {arg}"))
    }

    #[tool(description = "Tool with description attribute")]
    async fn tool_with_description_attr(&self) -> Result<&str> {
        Ok("description_attr")
    }

    /// Tool with doc comment
    #[tool(description = "Tool description from attribute")]
    async fn tool_description_priority(&self) -> Result<&str> {
        Ok("priority_test")
    }

    #[tool("custom_name", description = "Tool with name and description")]
    async fn tool_name_and_description(&self) -> Result<&str> {
        Ok("name_and_desc")
    }
}

#[test]
async fn list_some() -> Result<()> {
    let client = McpClient::with_server(MyMcpServer).await?;
    let a = client
        .tools_list(Some(ListToolsRequestParams::default()))
        .await?;
    let e = tools_expected()?;
    assert_eq!(a, e);
    Ok(())
}
#[test]
async fn list_none() -> Result<()> {
    let client = McpClient::with_server(MyMcpServer).await?;
    let a = client.tools_list(None).await?;
    let e = tools_expected()?;
    assert_eq!(a, e);
    Ok(())
}
fn tools_expected() -> Result<ListToolsResult> {
    Ok(vec![
        Tool::new("no_arg", ToolInputSchema::new()),
        Tool::new("xxx", ToolInputSchema::new()),
        Tool::new(
            "arg_1",
            ToolInputSchema::new().with_property::<String>("arg_1", "", true)?,
        ),
        Tool::new(
            "arg_2",
            ToolInputSchema::new()
                .with_property::<String>("arg_1", "", true)?
                .with_property::<String>("arg_2", "", true)?,
        ),
        Tool::new(
            "arg_opt",
            ToolInputSchema::new().with_property::<String>("arg_1", "", false)?,
        ),
        Tool::new(
            "arg_name_underscore",
            ToolInputSchema::new().with_property::<String>("arg", "", true)?,
        ),
        Tool::new(
            "arg_name_underscore_2",
            ToolInputSchema::new().with_property::<String>("_arg", "", true)?,
        ),
        Tool::new(
            "arg_attr_name",
            ToolInputSchema::new().with_property::<String>("xxx", "", true)?,
        ),
        Tool::new(
            "arg_attr_name_underscore",
            ToolInputSchema::new().with_property::<String>("_xxx", "", true)?,
        ),
        Tool::new("tool_description", ToolInputSchema::new()).with_description("Tool description"),
        Tool::new("tool_description_multiline", ToolInputSchema::new())
            .with_description("Tool description line1\nTool description line2"),
        Tool::new(
            "arg_description",
            ToolInputSchema::new().with_property::<String>("arg", "Arg description", true)?,
        ),
        Tool::new("tool_with_description_attr", ToolInputSchema::new())
            .with_description("Tool with description attribute"),
        Tool::new("tool_description_priority", ToolInputSchema::new())
            .with_description("Tool description from attribute"),
        Tool::new("custom_name", ToolInputSchema::new())
            .with_description("Tool with name and description"),
    ]
    .into())
}

#[test]
async fn call() -> Result<()> {
    let client = McpClient::with_server(MyMcpServer).await?;
    let a = client
        .tools_call(CallToolRequestParams::new("no_arg"))
        .await?;
    let e: CallToolResult = "abc".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn call_with_name() -> Result<()> {
    let client = McpClient::with_server(MyMcpServer).await?;
    let a = client
        .tools_call(CallToolRequestParams::new("xxx").with_argument("xxx", "abc")?)
        .await?;
    let e: CallToolResult = "def".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn call_with_arg_1() -> Result<()> {
    let client = McpClient::with_server(MyMcpServer).await?;
    let a = client
        .tools_call(CallToolRequestParams::new("arg_1").with_argument("arg_1", "abc")?)
        .await?;
    let e: CallToolResult = "abc".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn call_with_arg_2() -> Result<()> {
    let client = McpClient::with_server(MyMcpServer).await?;
    let a = client
        .tools_call(
            CallToolRequestParams::new("arg_2")
                .with_argument("arg_1", "abc")?
                .with_argument("arg_2", "def")?,
        )
        .await?;
    let e: CallToolResult = "abc def".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn call_with_arg_opt_some() -> Result<()> {
    let client = McpClient::with_server(MyMcpServer).await?;
    let a = client
        .tools_call(CallToolRequestParams::new("arg_opt").with_argument("arg_1", "abc")?)
        .await?;
    let e: CallToolResult = "abc".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn call_with_arg_opt_none() -> Result<()> {
    let client = McpClient::with_server(MyMcpServer).await?;
    let a = client
        .tools_call(CallToolRequestParams::new("arg_opt"))
        .await?;
    let e: CallToolResult = "---".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn call_with_arg_opt_null() -> Result<()> {
    let client = McpClient::with_server(MyMcpServer).await?;
    let a = client
        .tools_call(CallToolRequestParams::new("arg_opt").with_argument("arg_1", Value::Null)?)
        .await?;
    let e: CallToolResult = "---".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn call_missing_arg() -> Result<()> {
    let client = McpClient::with_server(MyMcpServer).await?;
    let a = client.tools_call(CallToolRequestParams::new("arg_1")).await;
    assert_error(a, ErrorCode::INVALID_PARAMS);
    Ok(())
}

#[test]
async fn call_name_mismatch() -> Result<()> {
    let client = McpClient::with_server(MyMcpServer).await?;
    let a = client
        .tools_call(CallToolRequestParams::new("unknown"))
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
