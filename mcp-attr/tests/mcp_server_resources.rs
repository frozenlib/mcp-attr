use pretty_assertions::assert_eq;
use tokio::test;

use mcp_attr::client::McpClient;
use mcp_attr::schema::{
    ListResourceTemplatesRequestParams, ListResourceTemplatesResult, ListResourcesRequestParams,
    ListResourcesResult, ReadResourceRequestParams, ReadResourceResult, Resource, ResourceTemplate,
};
use mcp_attr::server::{mcp_server, McpServer};
use mcp_attr::Result;

struct MyMcpServer;

#[mcp_server]
impl McpServer for MyMcpServer {
    #[resource("http://localhost/a.txt")]
    async fn no_arg(&self) -> Result<&str> {
        Ok("abc")
    }

    #[resource("http://localhost/b.txt", name = "xxx")]
    async fn no_arg_with_name(&self) -> Result<&str> {
        Ok("def")
    }

    #[resource("http://localhost/b/{a}", name = "xxx2")]
    async fn arg_with_name(&self, a: String) -> Result<String> {
        Ok(format!("hello {a}"))
    }

    #[resource("http://localhost/c.txt", mime_type = "text/plain")]
    async fn no_arg_with_mime_type(&self) -> Result<&str> {
        Ok("def")
    }

    #[resource("http://localhost/c/{a}", mime_type = "text/plain")]
    async fn arg_with_mime_type(&self, a: String) -> Result<String> {
        Ok(format!("hello {a}"))
    }

    #[resource("http://localhost/se/{a}")]
    async fn simple_expansion(&self, a: String) -> Result<String> {
        Ok(format!("hello {a}"))
    }

    #[resource("http://localhost/re/{+a}")]
    async fn reserved_expansion(&self, a: String) -> Result<String> {
        Ok(format!("hello {a}"))
    }

    #[resource("http://localhost/ge/{#a}")]
    async fn fragment_expansion(&self, a: String) -> Result<String> {
        Ok(format!("hello {a}"))
    }

    #[resource("http://localhost/a2/{a}/{b}")]
    async fn arg_2(&self, a: String, b: String) -> Result<String> {
        Ok(format!("hello {a} {b}"))
    }

    #[resource("http://localhost/ao_se/{#a}")]
    async fn arg_opt_simple_expansion(&self, a: Option<String>) -> Result<String> {
        if let Some(a) = a {
            Ok(format!("hello {a}"))
        } else {
            Ok("---".to_string())
        }
    }

    #[resource("http://localhost/ao_re/{+a}")]
    async fn arg_opt_reserved_expansion(&self, a: Option<String>) -> Result<String> {
        if let Some(a) = a {
            Ok(format!("hello {a}"))
        } else {
            Ok("---".to_string())
        }
    }

    #[resource("http://localhost/ao_fe/{#a}")]
    async fn arg_opt_fragment_expansion(&self, a: Option<String>) -> Result<String> {
        if let Some(a) = a {
            Ok(format!("hello {a}"))
        } else {
            Ok("---".to_string())
        }
    }

    #[resource("http://localhost/au/{arg}")]
    async fn arg_name_underscore(&self, _arg: String) -> Result<String> {
        Ok("---".to_string())
    }

    #[resource("http://localhost/au2/{_arg}")]
    async fn arg_name_underscore_2(&self, __arg: String) -> Result<String> {
        Ok("---".to_string())
    }

    /// Resource Description
    #[resource("http://localhost/rd")]
    async fn resource_description(&self) -> Result<&str> {
        Ok("resource_description")
    }

    /// Resource Template Description
    #[resource("http://localhost/rtd/{a}")]
    async fn resource_template_description(&self, a: String) -> Result<String> {
        Ok(format!("resource_template_description {a}"))
    }

    #[resource]
    async fn all_url(&self, url: String) -> Result<String> {
        Ok(format!("--{url}---"))
    }
}

#[test]
async fn list_some() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .resources_list(Some(ListResourcesRequestParams::default()))
        .await?;
    let e = resources_expected();
    assert_eq!(a, e);
    Ok(())
}
#[test]
async fn list_none() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client.resources_list(None).await?;
    let e = resources_expected();
    assert_eq!(a, e);
    Ok(())
}

fn resources_expected() -> ListResourcesResult {
    vec![
        Resource::new("http://localhost/a.txt", "no_arg"),
        Resource::new("http://localhost/b.txt", "xxx"),
        Resource::new("http://localhost/c.txt", "no_arg_with_mime_type")
            .with_mime_type("text/plain"),
        Resource::new("http://localhost/rd", "resource_description")
            .with_description("Resource Description"),
    ]
    .into()
}

#[test]
async fn templates_list_some() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .resources_templates_list(Some(ListResourceTemplatesRequestParams::default()))
        .await?;
    let e = templates_list_expected();
    assert_eq!(a, e);
    Ok(())
}
#[test]
async fn templates_list_none() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client.resources_templates_list(None).await?;
    let e = templates_list_expected();
    assert_eq!(a, e);
    Ok(())
}
fn templates_list_expected() -> ListResourceTemplatesResult {
    vec![
        ResourceTemplate::new("http://localhost/b/{a}", "xxx2"),
        ResourceTemplate::new("http://localhost/c/{a}", "arg_with_mime_type")
            .with_mime_type("text/plain"),
        ResourceTemplate::new("http://localhost/se/{a}", "simple_expansion"),
        ResourceTemplate::new("http://localhost/re/{+a}", "reserved_expansion"),
        ResourceTemplate::new("http://localhost/ge/{#a}", "fragment_expansion"),
        ResourceTemplate::new("http://localhost/a2/{a}/{b}", "arg_2"),
        ResourceTemplate::new("http://localhost/ao_se/{#a}", "arg_opt_simple_expansion"),
        ResourceTemplate::new("http://localhost/ao_re/{+a}", "arg_opt_reserved_expansion"),
        ResourceTemplate::new("http://localhost/ao_fe/{#a}", "arg_opt_fragment_expansion"),
        ResourceTemplate::new("http://localhost/au/{arg}", "arg_name_underscore"),
        ResourceTemplate::new("http://localhost/au2/{_arg}", "arg_name_underscore_2"),
        ResourceTemplate::new("http://localhost/rtd/{a}", "resource_template_description")
            .with_description("Resource Template Description"),
    ]
    .into()
}

#[test]
async fn read_no_arg() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .resources_read(ReadResourceRequestParams::new("http://localhost/a.txt"))
        .await?;
    let e: ReadResourceResult = "abc".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn read_no_arg_with_name() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .resources_read(ReadResourceRequestParams::new("http://localhost/b.txt"))
        .await?;
    let e: ReadResourceResult = "def".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn read_arg_with_name() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .resources_read(ReadResourceRequestParams::new("http://localhost/b/123"))
        .await?;
    let e: ReadResourceResult = "hello 123".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn read_simple_expansion() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .resources_read(ReadResourceRequestParams::new("http://localhost/se/123"))
        .await?;
    let e: ReadResourceResult = "hello 123".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn read_simple_expansion_decode() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .resources_read(ReadResourceRequestParams::new(
            "http://localhost/se/%E3%81%82",
        ))
        .await?;
    let e: ReadResourceResult = "hello あ".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn read_reserved_expansion() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .resources_read(ReadResourceRequestParams::new(
            "http://localhost/re/123/456",
        ))
        .await?;
    let e: ReadResourceResult = "hello 123/456".into();
    assert_eq!(a, e);
    Ok(())
}

#[test]
async fn read_reserved_expansion_not_decode() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .resources_read(ReadResourceRequestParams::new(
            "http://localhost/re/%E3%81%82",
        ))
        .await?;
    let e: ReadResourceResult = "hello %E3%81%82".into();
    assert_eq!(a, e);
    Ok(())
}
#[test]
async fn read_fragment_expansion() -> Result<()> {
    let client = McpClient::from_server(MyMcpServer).await?;
    let a = client
        .resources_read(ReadResourceRequestParams::new("http://localhost/ge/#123"))
        .await?;
    let e: ReadResourceResult = "hello 123".into();
    assert_eq!(a, e);
    Ok(())
}

// #[test]
// async fn read_simple_expansion_not_match() -> Result<()> {
//     let client = McpClient::from_server(MyMcpServer).await?;
//     let a = client
//         .resources_read(ReadResourceRequestParams::new(
//             "http://localhost/se/123/456",
//         ))
//         .await;
//     assert_error(a, ErrorCode::INVALID_PARAMS);
//     Ok(())
// }

// #[track_caller]
// fn assert_error<T: std::fmt::Debug>(a: SessionResult<T>, code: ErrorCode) {
//     match a {
//         Ok(_) => panic!("expected error.\n{a:#?}"),
//         Err(e) => {
//             if let Some(e) = e.error_object() {
//                 assert_eq!(e.code, code, "{e:#?}");
//             } else {
//                 panic!("no error object\n{e:#?}");
//             }
//         }
//     }
// }
