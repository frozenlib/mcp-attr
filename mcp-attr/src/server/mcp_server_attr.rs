// #[include_doc("../mcp_server_attr.md",start)]
/// [`crate::server::McpServer`] トレイトを実装する属性マクロ
///
/// `impl McpServer for X {}` に対して付与することで [`crate::server::McpServer`] を実装する。
///
/// ```rust
/// use mcp_attr::server::{mcp_server, McpServer};
///
/// struct X;
///
/// #[mcp_server]
/// impl McpServer for X {
///   // ...
/// }
/// ```
///
/// 下記の属性を付与した[`Result<T>`]を返す非同期メソッド実装することで model context protocol のメソッドを実装する。
///
/// 戻り値の型 `T` は付与する属性によって異なり、下記の表で示された型とその型に変換可能な型を使用できる。
///
/// | attribute                  | return type                           | model context protocol methods                                       |
/// | -------------------------- | ------------------------------------- | -------------------------------------------------------------------- |
/// | [`#[prompt]`](#prompt)     | [`crate::schema::GetPromptResult`]    | [`prompts/list`], [`prompts/get`]                                    |
/// | [`#[resource]`](#resource) | [`crate::schema::ReadResourceResult`] | [`resources/list`], [`resources/read`], [`resources/templates/list`] |
/// | [`#[tool]`](#tool)         | [`crate::schema::CallToolResult`]     | [`tools/list`], [`tools/call`]                                       |
///
/// # `#[prompt]`
///
/// ```rust,ignore
/// #[prompt("name")]
/// async fn func_name(&self) -> Result<GetPromptResult> { }
/// ```
///
/// - "name" (optional) : プロンプト名。省略した場合は関数名が使用される。
///
/// 下記のメソッドを実装する。
///
/// - [`prompts_list`]
/// - [`prompts_get`]
///
/// 関数の引数はプロンプトの引数となる。引数は [`FromStr`] の実装が必要。
///
/// 引数には `#[arg("name")]` 属性を付与することで名前を指定できる。
/// 指定しない場合は関数引数名の最初から `_` が取り除かれた名前が使用される。
///
/// 戻り値: [`Result<impl Into<crate::schema::GetPromptResult>>`](crate::schema::GetPromptResult)
///
/// ```rust
/// use mcp_attr::server::Result;
/// use mcp_attr::server::{mcp_server, McpServer};
///
/// struct X;
///
/// #[mcp_server]
/// impl McpServer for X {
///   /// 関数の説明 (AI用)
///   #[prompt]
///   async fn hello(&self) -> Result<&str> {
///     Ok("Hello, world!")
///   }
///
///   #[prompt]
///   async fn echo(&self,
///     /// 引数の説明 (AI用)
///     a: String,
///     /// 引数の説明 (AI用)
///     #[arg("x")]
///     b: String,
///   ) -> Result<String> {
///     Ok(format!("Hello, {a} {b}!"))
///   }
/// }
/// ```
///
/// # `#[resource]`
///
/// ```rust,ignore
/// #[resource("url_template", name = "name", mime_type = "mime_type")]
/// async fn func_name(&self) -> Result<ReadResourceResult> { }
/// ```
///
/// - "url_template" (optional) : このメソッドで処理するリソースの URL を示す URI Template([RFC 6570])。省略した場合は全ての URL を処理する。
/// - "name" (optional) : リソース名。省略した場合は関数名が使用される。
/// - "mime_type" (optional) : リソースの MIME タイプ。
///
/// 下記のメソッドを実装する。
///
/// - [`resources_list`] (手動実装可)
/// - [`resources_read`]
/// - [`resources_templates_list`]
///
/// 関数の引数は URI Template の変数となる。引数は [`FromStr`](std::str::FromStr) の実装が必要。
///
/// URI Template は [RFC 6570] Level2 に準拠。詳細は [`UriTemplate`] を参照のこと。
///
/// 戻り値: [`Result<impl Into<crate::schema::ReadResourceResult>>`](crate::schema::ReadResourceResult)
///
/// ```rust
/// use mcp_attr::server::Result;
/// use mcp_attr::server::{mcp_server, McpServer};
///
/// struct X;
///
/// #[mcp_server]
/// impl McpServer for X {
///   /// 関数の説明 (AI用)
/// #[resource("my_app://x/y.txt")]
///   async fn file_one(&self) -> Result<String> {
///     Ok(format!("one file"))
///   }
///
/// #[resource("my_app://{a}/{+b}")]
///   async fn file_ab(&self, a: String, b: String) -> Result<String> {
///     Ok(format!("{a} and {b}"))
///   }
///
///   #[resource]
///   async fn file_any(&self, url: String) -> Result<String> {
///     Ok(format!("any file"))
///   }
/// }
/// ```
///
/// 自動実装された [`resources_list`] は `#[resource]` 属性で指定された変数の無い URL の一覧を返す。
/// それ以外の URL を返す場合は `resources_list` を手動で実装する必要がある。
/// `resources_list` を手動で実装した場合は、`resources_list` は自動実装されない。
///
/// # `#[tool]`
///
/// ```rust,ignore
/// #[tool("name")]
/// async fn func_name(&self) -> Result<CallToolResult> { }
/// ```
///
/// - "name" (optional) : ツール名。省略した場合は関数名が使用される。
///
/// 下記のメソッドを実装する。
///
/// - [`tools_list`]
/// - [`tools_call`]
///
/// 関数の引数はツールの引数となる。引数は [`serde::de::DeserializeOwned`] + [`JsonSchema`] の実装が必要。
///
/// 引数には `#[arg("name")]` 属性を付与することで名前を指定できる。
/// 指定しない場合は関数引数名の最初から `_` が取り除かれた名前が使用される。
///
/// 戻り値: [`Result<impl Into<crate::schema::CallToolResult>>`](crate::schema::CallToolResult)
///
/// ```rust
/// use mcp_attr::server::Result;
/// use mcp_attr::server::{mcp_server, McpServer};
///
/// struct X;
///
/// #[mcp_server]
/// impl McpServer for X {
///   /// 関数の説明 (AI用)
///   #[tool]
///   async fn echo(&self,
///     /// 引数の説明 (AI用)
///     a: String,
///     /// 引数の説明 (AI用)
///     #[arg("x")]
///     b: String,
///   ) -> Result<String> {
///     Ok(format!("Hello, {a} {b}!"))
///   }
/// }
/// ```
///
/// [RFC 6570]: https://www.rfc-editor.org/rfc/rfc6570.html
/// [`prompts_list`]: crate::server::McpServer::prompts_list
/// [`prompts_get`]: crate::server::McpServer::prompts_get
/// [`resources_list`]: crate::server::McpServer::resources_list
/// [`resources_read`]: crate::server::McpServer::resources_read
/// [`resources_templates_list`]: crate::server::McpServer::resources_templates_list
/// [`tools_list`]: crate::server::McpServer::tools_list
/// [`tools_call`]: crate::server::McpServer::tools_call
/// [`prompts/list`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/prompts/#listing-prompts
/// [`prompts/get`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/prompts/#getting-a-prompt
/// [`resources/list`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/resources/#listing-resources
/// [`resources/read`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/resources/#reading-resources
/// [`resources/templates/list`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/resources/#resource-templates
/// [`tools/list`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/tools/#listing-tools
/// [`tools/call`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/tools/#calling-a-tool
/// [`FromStr`]: std::str::FromStr
/// [`JsonSchema`]: schemars::JsonSchema
/// [`UriTemplate`]: uri_template_ex::UriTemplate
// #[include_doc("../mcp_server_attr.md",end)]
pub use mcp_attrs_macros::mcp_server;
