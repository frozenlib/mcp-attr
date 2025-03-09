// #![include_doc("../../README.ja.md", start)]
//! # mcp-attr
//!
//! [![Crates.io](https://img.shields.io/crates/v/mcp-attr.svg)](https://crates.io/crates/mcp-attr)
//! [![Docs.rs](https://docs.rs/mcp-attr/badge.svg)](https://docs.rs/mcp-attr/)
//! [![Actions Status](https://github.com/frozenlib/mcp-attr/workflows/CI/badge.svg)](https://github.com/frozenlib/mcp-attr/actions)
//!
//! 属性と型を活用した宣言的な記述で Model Context Protocol サーバを作るための crate
//!
//! ## 特徴
//!
//! mcp-attr は人間と AI によって簡単に [Model Context Protocol] サーバを作れるようにする事を目的とした crate です。
//! この目的を達成する為、次のような特徴を持っています。
//!
//! - **宣言的な記述**:
//!   - `#[mcp_server]` を始めとする属性を使用することで、少ない行数で MCP サーバを記述できる
//!   - 行数が少ないので人間にとって理解しやすく、AI にとってもコンテキストウィンドウの消費が少ない
//!   - DRY(Don't Repeat Yourself) 原則に従っているので、矛盾のあるコードを防げる
//! - **型システムの活用**:
//!   - MCP クライアントに送信する情報を型で表現することによりソースコード量が減って可読性が高まる
//!   - 型エラーが AI によるコーディングの助けになる
//!
//! ## クイックスタート
//!
//! ### インストール
//!
//! `Cargo.toml`に以下を追加してください：
//!
//! ```toml
//! [dependencies]
//! mcp-attr = "0.0.2"
//! tokio = "1.43.0"
//! ```
//!
//! ### 基本的な使い方
//!
//! ```rust
//! use std::sync::Mutex;
//!
//! use mcp_attr::server::{mcp_server, McpServer, serve_stdio};
//! use mcp_attr::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     serve_stdio(ExampleServer(Mutex::new(ServerData { count: 0 }))).await?;
//!     Ok(())
//! }
//!
//! struct ExampleServer(Mutex<ServerData>);
//!
//! struct ServerData {
//!   /// サーバの状態
//!   count: u32,
//! }
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!     /// MCPクライアントに送信される解説
//!     #[prompt]
//!     async fn example_prompt(&self) -> Result<&str> {
//!         Ok("Hello!")
//!     }
//!
//!     #[resource("my_app://files/{name}.txt")]
//!     async fn read_file(&self, name: String) -> Result<String> {
//!         Ok(format!("Content of {name}.txt"))
//!     }
//!
//!     #[tool]
//!     async fn add_count(&self, message: String) -> Result<String> {
//!         let mut state = self.0.lock().unwrap();
//!         state.count += 1;
//!         Ok(format!("Echo: {message} {}", state.count))
//!     }
//! }
//! ```
//!
//! ### サポート状況
//!
//! 下記のプロトコルバージョン、トランスポート、メソッドをサポートしています。
//!
//! #### プロトコルバージョン
//!
//! - `2024-11-05`
//!
//! #### トランスポート
//!
//! - stdio
//!
//! SSE は未対応です。ただし、トランスポートは拡張可能なためカスタムトランスポートを実装することは可能です。
//!
//! #### メソッド
//!
//! | Attribute                  | [`McpServer`] methods                                                    | Model context protocol methods                                           |
//! | -------------------------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------ |
//! | [`#[prompt]`](#prompt)     | [`prompts_list`]<br>[`prompts_get`]                                      | [`prompts/list`]<br>[`prompts/get`]                                      |
//! | [`#[resource]`](#resource) | [`resources_list`]<br>[`resources_read`]<br>[`resources_templates_list`] | [`resources/list`]<br>[`resources/read`]<br>[`resources/templates/list`] |
//! | [`#[tool]`](#tool)         | [`tools_list`]<br>[`tools_call`]                                         | [`tools/list`]<br>[`tools/call`]                                         |
//!
//! ## 使い方
//!
//! ### サーバの開始
//!
//! この crate による Mcp サーバは非同期ランタイム tokio 上で動作します。
//!
//! `#[tokio::main]` で非同期ランタイムを起動し `serve_stdio` 関数に `McpServer` トレイトを実装した値を渡すことで
//! 標準入出力をトランスポートとするサーバが起動します。
//!
//! `McpServer` トレイトは手動で実装することもできますが、`#[mcp_server]` 属性を付与することで宣言的な方法で効率的に実装できます。
//!
//! ```rust
//! use mcp_attr::server::{mcp_server, McpServer, serve_stdio};
//! use mcp_attr::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!   serve_stdio(ExampleServer).await?;
//!   Ok(())
//! }
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   #[prompt]
//!   async fn hello(&self) -> Result<&str> {
//!     Ok("Hello, world!")
//!   }
//! }
//! ```
//!
//! また、MCP のメソッドを実装する関数はそのほとんどが非同期関数となっており、複数の関数が同時に実行されます。
//!
//! ### 入力と出力
//!
//! MCP サーバが MCP クライアントからどのようなデータを受け取るかは関数の引数の定義で表現されます。
//!
//! 例えば、次の例では `add` ツールは `lhs` と `rhs` という名前の整数を受け取ることを示します。
//! この情報は MCP サーバから MCP クライアントに送信され、MCP クライアントは適切な内容のデータをサーバに送信します。
//!
//! ```rust
//! use mcp_attr::server::{mcp_server, McpServer};
//! use mcp_attr::Result;
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   #[tool]
//!   async fn add(&self, lhs: u32, rhs: u32) -> Result<String> {
//!     Ok(format!("{}", lhs + rhs))
//!   }
//! }
//! ```
//!
//! 引数には使用できる型はメソッドによって異なり、下記のトレイトを実装する型を使用できます。
//!
//! | Attribute                  | Trait for argument types              | Return type            |
//! | -------------------------- | ------------------------------------- | ---------------------- |
//! | [`#[prompt]`](#prompt)     | [`FromStr`]                           | [`GetPromptResult`]    |
//! | [`#[resource]`](#resource) | [`FromStr`]                           | [`ReadResourceResult`] |
//! | [`#[tool]`](#tool)         | [`DeserializeOwned`] + [`JsonSchema`] | [`CallToolResult`]     |
//!
//! 引数は `Option<T>` を使用することもでき、その場合は必須でない引数として MCP クライアントに通知されます。
//!
//! 戻り値は上記の `Return type` の列で示された型に変換可能な型を `Result` でラップした型を使用できます。
//! 例えば、`CallToolResult` は `From<String>` を実装しているため、上の例のように `Result<String>` を戻り値として使用できます。
//!
//! ### AI 向けの説明
//!
//! MCP クライアントが MCP サーバのメソッドを呼び出すには、メソッドと引数の意味を AI が理解する必要があります。
//!
//! メソッドや引数にドキュメントコメントを付けると、この情報が MCP クライアントに送信され、AI はメソッドや引数の意味を理解できるようになります。
//!
//! ```rust
//! use mcp_attr::server::{mcp_server, McpServer};
//! use mcp_attr::Result;
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   /// ツールの解説
//!   #[tool]
//!   async fn concat(&self,
//!     /// 引数aの説明 (AI用)
//!     a: u32,
//!     /// 引数bの説明 (AI用)
//!     b: u32,
//!   ) -> Result<String> {
//!     Ok(format!("{a},{b}"))
//!   }
//! }
//! ```
//!
//! ### 状態の管理
//!
//! `McpServer` を実装する値は同時に実行される複数のメソッドで共有されるため `&self` のみ使用可能です。 `&mut self` は使用できません。
//!
//! 状態を持つには `Mutex` などの内部可変性を持つスレッドセーフな型を使用する必要があります。
//!
//! ```rust
//! use std::sync::Mutex;
//! use mcp_attr::server::{mcp_server, McpServer};
//! use mcp_attr::Result;
//!
//! struct ExampleServer(Mutex<ServerData>);
//! struct ServerData {
//!   count: u32,
//! }
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   #[tool]
//!   async fn add_count(&self) -> Result<String> {
//!     let mut state = self.0.lock().unwrap();
//!     state.count += 1;
//!     Ok(format!("count: {}", state.count))
//!   }
//! }
//! ```
//!
//! ### エラー処理
//!
//! mcp_attr では Rust の標準的なエラー処理方法である `Result` を使用します。
//!
//! エラー処理用の型として [`mcp_attr::Error`] と [`mcp_attr::Result`] (`std::result::Result<T, mcp_attr::Error>` の別名) が用意されています。
//!
//! `mcp_attr::Error` は [`anyhow::Error`] と似た、[`std::error::Error + Sync + Send + 'static`] を実装する任意のエラー型を格納できる型で、他のエラー型からの変換が実装されています。
//! そのため `mcp_attr::Result` を返す関数では、型が `Result<T, impl std::error::Error + Sync + Send + 'static>` となる式に `?` 演算子を使用してエラー処理を行う事ができます。
//!
//! ただし、`anyhow::Error` とは下記の点が異なります。
//!
//! - MCP で使用されている JSON-RPC のエラーを格納できる
//! - エラーメッセージが MCP Client に送信する公開情報であるか、送信しないプライベート情報であるかを区別する機能を持つ
//!   - (ただし、デバッグビルドでは全ての情報が MCP Client に送信される)
//!
//! [`anyhow::bail!`] のようなエラー処理用のマクロとして [`bail!`] と [`bail_public!`] が用意されています。
//!
//! - [`bail!`] はフォーマット文字列と引数を取り、非公開情報として扱われるエラーを発生させます。
//! - [`bail_public!`] はエラーコードとフォーマット文字列、引数を取り、公開情報として扱われるエラーを発生させます。
//!
//! また、他のエラー型からの変換は非公開情報として扱われます。
//!
//! ```rust
//! use mcp_attr::server::{mcp_server, McpServer};
//! use mcp_attr::{bail, bail_public, Result, ErrorCode};
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!     #[prompt]
//!     async fn add(&self, a: String) -> Result<String> {
//!         let something_wrong = false;
//!         if something_wrong {
//!             bail_public!(ErrorCode::INTERNAL_ERROR, "エラーメッセージ");
//!         }
//!         if something_wrong {
//!             bail!("エラーメッセージ");
//!         }
//!         let a = a.parse::<i32>()?;
//!         Ok(format!("成功 {a}"))
//!     }
//! }
//! ```
//!
//! ## 各属性の説明
//!
//! ### `#[prompt]`
//!
//! ```rust,ignore
//! #[prompt("name")]
//! async fn func_name(&self) -> Result<GetPromptResult> { }
//! ```
//!
//! - "name" (optional) : プロンプト名。省略した場合は関数名が使用される。
//!
//! 下記のメソッドを実装する。
//!
//! - [`prompts_list`]
//! - [`prompts_get`]
//!
//! 関数の引数はプロンプトの引数となる。引数は下記のトレイトの実装が必要。
//!
//! - [`FromStr`] : 文字列から値を復元する為のトレイト
//!
//! 引数には `#[arg("name")]` 属性を付与することで名前を指定できる。
//! 指定しない場合は関数引数名の最初から `_` が取り除かれた名前が使用される。
//!
//! 戻り値: [`Result<impl Into<GetPromptResult>>`](GetPromptResult)
//!
//! ```rust
//! use mcp_attr::server::Result;
//! use mcp_attr::server::{mcp_server, McpServer};
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   /// 関数の説明 (AI用)
//!   #[prompt]
//!   async fn hello(&self) -> Result<&str> {
//!     Ok("Hello, world!")
//!   }
//!
//!   #[prompt]
//!   async fn echo(&self,
//!     /// 引数の説明 (AI用)
//!     a: String,
//!     /// 引数の説明 (AI用)
//!     #[arg("x")]
//!     b: String,
//!   ) -> Result<String> {
//!     Ok(format!("Hello, {a} {b}!"))
//!   }
//! }
//! ```
//!
//! ### `#[resource]`
//!
//! ```rust,ignore
//! #[resource("url_template", name = "name", mime_type = "mime_type")]
//! async fn func_name(&self) -> Result<ReadResourceResult> { }
//! ```
//!
//! - "url_template" (optional) : このメソッドで処理するリソースの URL を示す URI Template([RFC 6570])。省略した場合は全ての URL を処理する。
//! - "name" (optional) : リソース名。省略した場合は関数名が使用される。
//! - "mime_type" (optional) : リソースの MIME タイプ。
//!
//! 下記のメソッドを実装する。
//!
//! - [`resources_list`] (手動実装可)
//! - [`resources_read`]
//! - [`resources_templates_list`]
//!
//! 関数の引数は URI Template の変数となる。引数は下記のトレイトの実装が必要。
//!
//! - [`FromStr`] : 文字列から値を復元する為のトレイト
//!
//! URI Template は [RFC 6570] Level2 で指定。下記の 3 種類の変数が使用できる。
//!
//! - `{var}`
//! - `{+var}`
//! - `{#var}`
//!
//! 戻り値: [`Result<impl Into<ReadResourceResult>>`](ReadResourceResult)
//!
//! ```rust
//! use mcp_attr::server::Result;
//! use mcp_attr::server::{mcp_server, McpServer};
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   /// 関数の説明 (AI用)
//!   #[resource("my_app://x/y.txt")]
//!   async fn file_one(&self) -> Result<String> {
//!     Ok(format!("one file"))
//!   }
//!
//!   #[resource("my_app://{a}/{+b}")]
//!   async fn file_ab(&self, a: String, b: String) -> Result<String> {
//!     Ok(format!("{a} and {b}"))
//!   }
//!
//!   #[resource]
//!   async fn file_any(&self, url: String) -> Result<String> {
//!     Ok(format!("any file"))
//!   }
//! }
//! ```
//!
//! 自動実装された [`resources_list`] は `#[resource]` 属性で指定された変数の無い URL の一覧を返す。
//! それ以外の URL を返す場合は `resources_list` を手動で実装する必要がある。
//! `resources_list` を手動で実装した場合は、`resources_list` は自動実装されない。
//!
//! ### `#[tool]`
//!
//! ```rust,ignore
//! #[tool("name")]
//! async fn func_name(&self) -> Result<CallToolResult> { }
//! ```
//!
//! - "name" (optional) : ツール名。省略した場合は関数名が使用される。
//!
//! 下記のメソッドを実装する。
//!
//! - [`tools_list`]
//! - [`tools_call`]
//!
//! 関数の引数はツールの引数となる。引数は下記のすべてのトレイトの実装が必要の実装が必要。
//!
//! - [`DeserializeOwned`]: JSON から値を復元する為のトレイト
//! - [`JsonSchema`]: JSON Schema を生成する為のトレイト（JSON Schema は MCP Client に送信され、AI が引数の構造を理解できるようになる）
//!
//! 引数には `#[arg("name")]` 属性を付与することで名前を指定できる。
//! 指定しない場合は関数引数名の最初から `_` が取り除かれた名前が使用される。
//!
//! 戻り値: [`Result<impl Into<CallToolResult>>`](CallToolResult)
//!
//! ```rust
//! use mcp_attr::server::Result;
//! use mcp_attr::server::{mcp_server, McpServer};
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   /// 関数の説明 (AI用)
//!   #[tool]
//!   async fn echo(&self,
//!     /// 引数の説明 (AI用)
//!     a: String,
//!     /// 引数の説明 (AI用)
//!     #[arg("x")]
//!     b: String,
//!   ) -> Result<String> {
//!     Ok(format!("Hello, {a} {b}!"))
//!   }
//! }
//! ```
//!
//! ### 手動実装
//!
//! 属性を使用せず `McpServer` のメソッドを直接実装することもできます。
//!
//! また、下記のメソッドは属性による実装に対応しておらず、手動での実装のみが可能です。
//!
//! - [`server_info`]
//! - [`instructions`]
//! - [`completion_complete`]
//!
//! 次のメソッドは、属性による実装を手動での実装で上書きすることができます。
//!
//! - [`tools_list`]
//!
//! ### テスト方法
//!
//! AI Coding Agent の登場によりテストはより重要になりました。
//! AI はテスト無しでは、正しいコードをほとんど書く事ができませんが、テストがあればテストと修正を繰り返すことで正しいコードを書くことができるでしょう。
//!
//! mcp_attr には、プロセス内で MCP サーバと接続するテスト用の [`McpClient`] が含まれています。
//!
//! ```rust
//! use mcp_attr::client::McpClient;
//! use mcp_attr::server::{mcp_server, McpServer};
//! use mcp_attr::schema::{GetPromptRequestParams, GetPromptResult};
//! use mcp_attr::Result;
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!     #[prompt]
//!     async fn hello(&self) -> Result<&str> {
//!         Ok("Hello, world!")
//!     }
//! }
//!
//! #[tokio::test]
//! async fn test_hello() -> Result<()> {
//!     let client = McpClient::from_server(ExampleServer).await?;
//!     let a = client
//!         .prompts_get(GetPromptRequestParams::new("hello"))
//!         .await?;
//!     let e: GetPromptResult = "Hello, world!".into();
//!     assert_eq!(a, e);
//!     Ok(())
//! }
//! ```
//!
//! ## License
//!
//! This project is dual licensed under Apache-2.0/MIT. See the two LICENSE-\* files for details.
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
//!
//! [Model Context Protocol]: https://spec.modelcontextprotocol.io/specification/2024-11-05/
//! [RFC 6570]: https://www.rfc-editor.org/rfc/rfc6570.html
//! [`prompts/list`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/prompts/#listing-prompts
//! [`prompts/get`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/prompts/#getting-a-prompt
//! [`resources/list`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/resources/#listing-resources
//! [`resources/read`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/resources/#reading-resources
//! [`resources/templates/list`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/resources/#resource-templates
//! [`tools/list`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/tools/#listing-tools
//! [`tools/call`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/tools/#calling-a-tool
//! [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
//! [`JsonSchema`]: https://docs.rs/schemars/latest/schemars/trait.JsonSchema.html
//! [`DeserializeOwned`]: https://docs.rs/serde/latest/serde/de/trait.DeserializeOwned.html
//! [`McpServer`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html
//! [`McpClient`]: https://docs.rs/mcp-attr/latest/mcp_attr/client/struct.McpClient.html
//! [`prompts_list`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.prompts_list
//! [`prompts_get`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.prompts_get
//! [`resources_list`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.resources_list
//! [`resources_read`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.resources_read
//! [`resources_templates_list`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.resources_templates_list
//! [`tools_list`]: https://docs.rs/mcp-attr/latest/mcp_attr/client/struct.McpClient.html#method.tools_list
//! [`tools_call`]: https://docs.rs/mcp-attr/latest/mcp_attr/client/struct.McpClient.html#method.tools_call
//! [`GetPromptResult`]: https://docs.rs/mcp-attr/latest/mcp_attr/schema/struct.GetPromptResult.html
//! [`ReadResourceResult`]: https://docs.rs/mcp-attr/latest/mcp_attr/schema/struct.ReadResourceResult.html
//! [`CallToolResult`]: https://docs.rs/mcp-attr/latest/mcp_attr/schema/struct.CallToolResult.html
//! [`mcp_attr::Error`]: https://docs.rs/mcp-attr/latest/mcp_attr/struct.Error.html
//! [`mcp_attr::Result`]: https://docs.rs/mcp-attr/latest/mcp_attr/type.Result.html
//! [`anyhow::Error`]: https://docs.rs/anyhow/latest/anyhow/struct.Error.html
//! [`std::error::Error + Sync + Send + 'static`]: https://doc.rust-lang.org/std/error/trait.Error.html
//! [`anyhow::bail!`]: https://docs.rs/anyhow/latest/anyhow/macro.bail.html
//! [`bail!`]: https://docs.rs/mcp-attr/latest/mcp_attr/macro.bail.html
//! [`bail_public!`]: https://docs.rs/mcp-attr/latest/mcp_attr/macro.bail_public.html
//! [`server_info`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.server_info
//! [`instructions`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.instructions
//! [`completion_complete`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.completion_complete
// #![include_doc("../../README.ja.md", end)]
// #![include_doc("../../README.md", start)]
//! # mcp-attr
//!
//! [![Crates.io](https://img.shields.io/crates/v/mcp-attr.svg)](https://crates.io/crates/mcp-attr)
//! [![Docs.rs](https://docs.rs/mcp-attr/badge.svg)](https://docs.rs/mcp-attr/)
//! [![Actions Status](https://github.com/frozenlib/mcp-attr/workflows/CI/badge.svg)](https://github.com/frozenlib/mcp-attr/actions)
//!
//! A crate for creating Model Context Protocol servers using declarative descriptions with attributes and types.
//!
//! ## Features
//!
//! mcp-attr is a crate designed to make it easy for both humans and AI to create [Model Context Protocol] servers.
//! To achieve this goal, it has the following features:
//!
//! - **Declarative Description**:
//!   - Use attributes like `#[mcp_server]` to describe MCP servers with minimal code
//!   - Fewer lines of code make it easier for humans to understand and consume less context window for AI
//!   - Following the DRY (Don't Repeat Yourself) principle helps prevent inconsistent code
//! - **Leveraging the Type System**:
//!   - Expressing information sent to MCP clients through types reduces source code volume and improves readability
//!   - Type errors help AI with coding
//!
//! ## Quick Start
//!
//! ### Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! mcp-attr = "0.0.2"
//! tokio = "1.43.0"
//! ```
//!
//! ### Basic Usage
//!
//! ```rust
//! use std::sync::Mutex;
//!
//! use mcp_attr::server::{mcp_server, McpServer, serve_stdio};
//! use mcp_attr::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     serve_stdio(ExampleServer(Mutex::new(ServerData { count: 0 }))).await?;
//!     Ok(())
//! }
//!
//! struct ExampleServer(Mutex<ServerData>);
//!
//! struct ServerData {
//!   /// Server state
//!   count: u32,
//! }
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!     /// Description sent to MCP client
//!     #[prompt]
//!     async fn example_prompt(&self) -> Result<&str> {
//!         Ok("Hello!")
//!     }
//!
//!     #[resource("my_app://files/{name}.txt")]
//!     async fn read_file(&self, name: String) -> Result<String> {
//!         Ok(format!("Content of {name}.txt"))
//!     }
//!
//!     #[tool]
//!     async fn add_count(&self, message: String) -> Result<String> {
//!         let mut state = self.0.lock().unwrap();
//!         state.count += 1;
//!         Ok(format!("Echo: {message} {}", state.count))
//!     }
//! }
//! ```
//!
//! ### Support Status
//!
//! The following protocol versions, transports, and methods are supported.
//!
//! #### Protocol Versions
//!
//! - `2024-11-05`
//!
//! #### Transport
//!
//! - stdio
//!
//! SSE is not yet supported. However, transport is extensible, so custom transports can be implemented.
//!
//! #### Methods
//!
//! | Attribute                  | [`McpServer`] methods                                                    | Model context protocol methods                                           |
//! | -------------------------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------ |
//! | [`#[prompt]`](#prompt)     | [`prompts_list`]<br>[`prompts_get`]                                      | [`prompts/list`]<br>[`prompts/get`]                                      |
//! | [`#[resource]`](#resource) | [`resources_list`]<br>[`resources_read`]<br>[`resources_templates_list`] | [`resources/list`]<br>[`resources/read`]<br>[`resources/templates/list`] |
//! | [`#[tool]`](#tool)         | [`tools_list`]<br>[`tools_call`]                                         | [`tools/list`]<br>[`tools/call`]                                         |
//!
//! ## Usage
//!
//! ### Starting the Server
//!
//! MCP servers created with this crate run on the tokio async runtime.
//!
//! Start the server by launching the async runtime with `#[tokio::main]` and passing a value implementing the `McpServer` trait to the `serve_stdio` function,
//! which starts a server using standard input/output as transport.
//!
//! While you can implement the `McpServer` trait manually, you can implement it more efficiently in a declarative way by using the `#[mcp_server]` attribute.
//!
//! ```rust
//! use mcp_attr::server::{mcp_server, McpServer, serve_stdio};
//! use mcp_attr::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!   serve_stdio(ExampleServer).await?;
//!   Ok(())
//! }
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   #[prompt]
//!   async fn hello(&self) -> Result<&str> {
//!     Ok("Hello, world!")
//!   }
//! }
//! ```
//!
//! Most of the functions implementing MCP methods are asynchronous and can be executed concurrently.
//!
//! ### Input and Output
//!
//! How an MCP server receives data from an MCP client is expressed through function argument definitions.
//!
//! For example, in the following example, the `add` tool indicates that it receives integers named `lhs` and `rhs`.
//! This information is sent from the MCP server to the MCP client, and the MCP client sends appropriate data to the server.
//!
//! ```rust
//! use mcp_attr::server::{mcp_server, McpServer};
//! use mcp_attr::Result;
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   #[tool]
//!   async fn add(&self, lhs: u32, rhs: u32) -> Result<String> {
//!     Ok(format!("{}", lhs + rhs))
//!   }
//! }
//! ```
//!
//! The types that can be used for arguments vary by method, and must implement the following traits:
//!
//! | Attribute                  | Trait for argument types              | Return type            |
//! | -------------------------- | ------------------------------------- | ---------------------- |
//! | [`#[prompt]`](#prompt)     | [`FromStr`]                           | [`GetPromptResult`]    |
//! | [`#[resource]`](#resource) | [`FromStr`]                           | [`ReadResourceResult`] |
//! | [`#[tool]`](#tool)         | [`DeserializeOwned`] + [`JsonSchema`] | [`CallToolResult`]     |
//!
//! Arguments can also use `Option<T>`, in which case they are communicated to the MCP client as optional arguments.
//!
//! Return values must be types that can be converted to the type shown in the `Return type` column above, wrapped in `Result`.
//! For example, since `CallToolResult` implements `From<String>`, you can use `Result<String>` as the return value as shown in the example above.
//!
//! ### Explanations for AI
//!
//! For an MCP client to call MCP server methods, the AI needs to understand the meaning of the methods and arguments.
//!
//! Adding documentation comments to methods and arguments sends this information to the MCP client, allowing the AI to understand their meaning.
//!
//! ```rust
//! use mcp_attr::server::{mcp_server, McpServer};
//! use mcp_attr::Result;
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   /// Tool description
//!   #[tool]
//!   async fn concat(&self,
//!     /// Description of argument a (for AI)
//!     a: u32,
//!     /// Description of argument b (for AI)
//!     b: u32,
//!   ) -> Result<String> {
//!     Ok(format!("{a},{b}"))
//!   }
//! }
//! ```
//!
//! ### State Management
//!
//! Since values implementing `McpServer` are shared among multiple concurrently executing methods, only `&self` is available. `&mut self` cannot be used.
//!
//! To maintain state, you need to use thread-safe types with interior mutability like `Mutex`.
//!
//! ```rust
//! use std::sync::Mutex;
//! use mcp_attr::server::{mcp_server, McpServer};
//! use mcp_attr::Result;
//!
//! struct ExampleServer(Mutex<ServerData>);
//! struct ServerData {
//!   count: u32,
//! }
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   #[tool]
//!   async fn add_count(&self) -> Result<String> {
//!     let mut state = self.0.lock().unwrap();
//!     state.count += 1;
//!     Ok(format!("count: {}", state.count))
//!   }
//! }
//! ```
//!
//! ### Error Handling
//!
//! mcp_attr uses `Result`, Rust's standard error handling method.
//!
//! The types [`mcp_attr::Error`] and [`mcp_attr::Result`] (an alias for `std::result::Result<T, mcp_attr::Error>`) are provided for error handling.
//!
//! `mcp_attr::Error` is similar to [`anyhow::Error`], capable of storing any error type implementing [`std::error::Error + Sync + Send + 'static`], and implements conversion from other error types.
//! Therefore, in functions returning `mcp_attr::Result`, you can use the `?` operator for error handling with expressions of type `Result<T, impl std::error::Error + Sync + Send + 'static>`.
//!
//! However, it differs from `anyhow::Error` in the following ways:
//!
//! - Can store JSON-RPC errors used in MCP
//! - Has functionality to distinguish whether error messages are public information to be sent to the MCP Client or private information not to be sent
//!   - (However, in debug builds, all information is sent to the MCP Client)
//!
//! The macros [`bail!`] and [`bail_public!`] are provided for error handling, similar to [`anyhow::bail!`].
//!
//! - [`bail!`] takes a format string and arguments and raises an error treated as private information.
//! - [`bail_public!`] takes an error code, format string, and arguments and raises an error treated as public information.
//!
//! Additionally, conversions from other error types are treated as private information.
//!
//! ```rust
//! use mcp_attr::server::{mcp_server, McpServer};
//! use mcp_attr::{bail, bail_public, Result, ErrorCode};
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!     #[prompt]
//!     async fn add(&self, a: String) -> Result<String> {
//!         let something_wrong = false;
//!         if something_wrong {
//!             bail_public!(ErrorCode::INTERNAL_ERROR, "error message");
//!         }
//!         if something_wrong {
//!             bail!("error message");
//!         }
//!         let a = a.parse::<i32>()?;
//!         Ok(format!("success {a}"))
//!     }
//! }
//! ```
//!
//! ## Attribute Descriptions
//!
//! ### `#[prompt]`
//!
//! ```rust,ignore
//! #[prompt("name")]
//! async fn func_name(&self) -> Result<GetPromptResult> { }
//! ```
//!
//! - "name" (optional): Prompt name. If omitted, the function name is used.
//!
//! Implements the following methods:
//!
//! - [`prompts_list`]
//! - [`prompts_get`]
//!
//! Function arguments become prompt arguments. Arguments must implement the following trait:
//!
//! - [`FromStr`]: Trait for restoring values from strings
//!
//! Arguments can be given names using the `#[arg("name")]` attribute.
//! If not specified, the name used is the function argument name with leading `_` removed.
//!
//! Return value: [`Result<impl Into<GetPromptResult>>`](GetPromptResult)
//!
//! ```rust
//! use mcp_attr::server::Result;
//! use mcp_attr::server::{mcp_server, McpServer};
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   /// Function description (for AI)
//!   #[prompt]
//!   async fn hello(&self) -> Result<&str> {
//!     Ok("Hello, world!")
//!   }
//!
//!   #[prompt]
//!   async fn echo(&self,
//!     /// Argument description (for AI)
//!     a: String,
//!     /// Argument description (for AI)
//!     #[arg("x")]
//!     b: String,
//!   ) -> Result<String> {
//!     Ok(format!("Hello, {a} {b}!"))
//!   }
//! }
//! ```
//!
//! ### `#[resource]`
//!
//! ```rust,ignore
//! #[resource("url_template", name = "name", mime_type = "mime_type")]
//! async fn func_name(&self) -> Result<ReadResourceResult> { }
//! ```
//!
//! - "url_template" (optional): URI Template ([RFC 6570]) indicating the URL of resources this method handles. If omitted, handles all URLs.
//! - "name" (optional): Resource name. If omitted, the function name is used.
//! - "mime_type" (optional): MIME type of the resource.
//!
//! Implements the following methods:
//!
//! - [`resources_list`] (can be manually implemented)
//! - [`resources_read`]
//! - [`resources_templates_list`]
//!
//! Function arguments become URI Template variables. Arguments must implement the following trait:
//!
//! - [`FromStr`]: Trait for restoring values from strings
//!
//! URI Templates are specified in [RFC 6570] Level2. The following variables can be used in URI Templates:
//!
//! - `{var}`
//! - `{+var}`
//! - `{#var}`
//!
//! Return value: [`Result<impl Into<ReadResourceResult>>`](ReadResourceResult)
//!
//! ```rust
//! use mcp_attr::server::Result;
//! use mcp_attr::server::{mcp_server, McpServer};
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   /// Function description (for AI)
//!   #[resource("my_app://x/y.txt")]
//!   async fn file_one(&self) -> Result<String> {
//!     Ok(format!("one file"))
//!   }
//!
//!   #[resource("my_app://{a}/{+b}")]
//!   async fn file_ab(&self, a: String, b: String) -> Result<String> {
//!     Ok(format!("{a} and {b}"))
//!   }
//!
//!   #[resource]
//!   async fn file_any(&self, url: String) -> Result<String> {
//!     Ok(format!("any file"))
//!   }
//! }
//! ```
//!
//! The automatically implemented [`resources_list`] returns a list of URLs without variables specified in the `#[resource]` attribute.
//! If you need to return other URLs, you must manually implement `resources_list`.
//! If `resources_list` is manually implemented, it is not automatically implemented.
//!
//! ### `#[tool]`
//!
//! ```rust,ignore
//! #[tool("name")]
//! async fn func_name(&self) -> Result<CallToolResult> { }
//! ```
//!
//! - "name" (optional): Tool name. If omitted, the function name is used.
//!
//! Implements the following methods:
//!
//! - [`tools_list`]
//! - [`tools_call`]
//!
//! Function arguments become tool arguments. Arguments must implement all of the following traits:
//!
//! - [`DeserializeOwned`]: Trait for restoring values from JSON
//! - [`JsonSchema`]: Trait for generating JSON Schema (JSON Schema is sent to MCP Client so AI can understand argument structure)
//!
//! Arguments can be given names using the `#[arg("name")]` attribute.
//! If not specified, the name used is the function argument name with leading `_` removed.
//!
//! Return value: [`Result<impl Into<CallToolResult>>`](CallToolResult)
//!
//! ```rust
//! use mcp_attr::server::Result;
//! use mcp_attr::server::{mcp_server, McpServer};
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   /// Function description (for AI)
//!   #[tool]
//!   async fn echo(&self,
//!     /// Argument description (for AI)
//!     a: String,
//!     /// Argument description (for AI)
//!     #[arg("x")]
//!     b: String,
//!   ) -> Result<String> {
//!     Ok(format!("Hello, {a} {b}!"))
//!   }
//! }
//! ```
//!
//! ### Manual Implementation
//!
//! You can also directly implement `McpServer` methods without using attributes.
//!
//! Additionally, the following methods do not support implementation through attributes and must be implemented manually:
//!
//! - [`server_info`]
//! - [`instructions`]
//! - [`completion_complete`]
//!
//! The following method can be overridden with manual implementation over the attribute-based implementation:
//!
//! - [`tools_list`]
//!
//! ### Testing
//!
//! With the advent of AI Coding Agents, testing has become even more important.
//! AI can hardly write correct code without tests, but with tests, it can write correct code through repeated testing and fixes.
//!
//! mcp_attr includes [`McpClient`] for testing, which connects to MCP servers within the process.
//!
//! ```rust
//! use mcp_attr::client::McpClient;
//! use mcp_attr::server::{mcp_server, McpServer};
//! use mcp_attr::schema::{GetPromptRequestParams, GetPromptResult};
//! use mcp_attr::Result;
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!     #[prompt]
//!     async fn hello(&self) -> Result<&str> {
//!         Ok("Hello, world!")
//!     }
//! }
//!
//! #[tokio::test]
//! async fn test_hello() -> Result<()> {
//!     let client = McpClient::from_server(ExampleServer).await?;
//!     let a = client
//!         .prompts_get(GetPromptRequestParams::new("hello"))
//!         .await?;
//!     let e: GetPromptResult = "Hello, world!".into();
//!     assert_eq!(a, e);
//!     Ok(())
//! }
//! ```
//!
//! ## License
//!
//! This project is dual licensed under Apache-2.0/MIT. See the two LICENSE-\* files for details.
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
//!
//! [Model Context Protocol]: https://spec.modelcontextprotocol.io/specification/2024-11-05/
//! [RFC 6570]: https://www.rfc-editor.org/rfc/rfc6570.html
//! [`prompts/list`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/prompts/#listing-prompts
//! [`prompts/get`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/prompts/#getting-a-prompt
//! [`resources/list`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/resources/#listing-resources
//! [`resources/read`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/resources/#reading-resources
//! [`resources/templates/list`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/resources/#resource-templates
//! [`tools/list`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/tools/#listing-tools
//! [`tools/call`]: https://spec.modelcontextprotocol.io/specification/2024-11-05/server/tools/#calling-a-tool
//! [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
//! [`JsonSchema`]: https://docs.rs/schemars/latest/schemars/trait.JsonSchema.html
//! [`DeserializeOwned`]: https://docs.rs/serde/latest/serde/de/trait.DeserializeOwned.html
//! [`McpServer`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html
//! [`McpClient`]: https://docs.rs/mcp-attr/latest/mcp_attr/client/struct.McpClient.html
//! [`prompts_list`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.prompts_list
//! [`prompts_get`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.prompts_get
//! [`resources_list`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.resources_list
//! [`resources_read`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.resources_read
//! [`resources_templates_list`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.resources_templates_list
//! [`tools_list`]: https://docs.rs/mcp-attr/latest/mcp_attr/client/struct.McpClient.html#method.tools_list
//! [`tools_call`]: https://docs.rs/mcp-attr/latest/mcp_attr/client/struct.McpClient.html#method.tools_call
//! [`GetPromptResult`]: https://docs.rs/mcp-attr/latest/mcp_attr/schema/struct.GetPromptResult.html
//! [`ReadResourceResult`]: https://docs.rs/mcp-attr/latest/mcp_attr/schema/struct.ReadResourceResult.html
//! [`CallToolResult`]: https://docs.rs/mcp-attr/latest/mcp_attr/schema/struct.CallToolResult.html
//! [`mcp_attr::Error`]: https://docs.rs/mcp-attr/latest/mcp_attr/struct.Error.html
//! [`mcp_attr::Result`]: https://docs.rs/mcp-attr/latest/mcp_attr/type.Result.html
//! [`anyhow::Error`]: https://docs.rs/anyhow/latest/anyhow/struct.Error.html
//! [`std::error::Error + Sync + Send + 'static`]: https://doc.rust-lang.org/std/error/trait.Error.html
//! [`anyhow::bail!`]: https://docs.rs/anyhow/latest/anyhow/macro.bail.html
//! [`bail!`]: https://docs.rs/mcp-attr/latest/mcp_attr/macro.bail.html
//! [`bail_public!`]: https://docs.rs/mcp-attr/latest/mcp_attr/macro.bail_public.html
//! [`server_info`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.server_info
//! [`instructions`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.instructions
//! [`completion_complete`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/trait.McpServer.html#method.completion_complete
// #![include_doc("../../README.md", end)]
