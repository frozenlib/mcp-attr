// #![include_doc("../../README.ja.md", start)]
//! # mcp-attr
//!
//! [![Crates.io](https://img.shields.io/crates/v/mcp-attr.svg)](https://crates.io/crates/mcp-attr)
//! [![Docs.rs](https://docs.rs/mcp-attr/badge.svg)](https://docs.rs/mcp-attr/)
//! [![Actions Status](https://github.com/frozenlib/mcp-attr/workflows/CI/badge.svg)](https://github.com/frozenlib/mcp-attr/actions)
//!
//! 宣言的な記述で Model Context Protocol サーバを作るためのライブラリ
//!
//! ## 特徴
//!
//! mcp-attr は人間と AI によって簡単に [Model Context Protocol] サーバを作れるようにする事を目的とした crate です。
//! この目的を達成する為、次のような特徴を持っています。
//!
//! - **宣言的な記述**:
//!   - `#[mcp_server]` を始めとする属性を使用することで、少ない行数で MCP サーバを記述できる
//!   - 行数が少ないので人間にとって理解しやすく、AI にとってもコンテキストウィンドウの消費が少ない
//! - **DRY(Don't Repeat Yourself) 原則**:
//!   - 宣言的な記述により DRY 原則に従ったコードを実現
//!   - AI が矛盾のあるコードを書く事を防ぐ
//! - **型システムの活用**:
//!   - MCP クライアントに送信する情報を型で表現することによりソースコード量が減り、可読性が高まる
//!   - 型エラーが AI によるコーディングの助けになる
//! - **`rustfmt` との親和性**:
//!   - マクロは `rustfmt` によるフォーマットが適用される属性マクロのみを利用
//!   - AI によって生成されたコードを確実に整形できる
//!
//! ## クイックスタート
//!
//! ### インストール
//!
//! `Cargo.toml`に以下を追加してください：
//!
//! ```toml
//! [dependencies]
//! mcp-attr = "0.0.7"
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
//!     #[tool]
//!     async fn add_count(&self, message: String) -> Result<String> {
//!         let mut state = self.0.lock().unwrap();
//!         state.count += 1;
//!         Ok(format!("Echo: {message} {}", state.count))
//!     }
//!
//!     #[resource("my_app://files/{name}.txt")]
//!     async fn read_file(&self, name: String) -> Result<String> {
//!         Ok(format!("Content of {name}.txt"))
//!     }
//!
//!     #[prompt]
//!     async fn example_prompt(&self) -> Result<&str> {
//!         Ok("Hello!")
//!     }
//! }
//! ```
//!
//! ## サポート状況
//!
//! 下記のプロトコルバージョン、トランスポート、メソッドをサポートしています。
//!
//! ### プロトコルバージョン
//!
//! - `2025-03-26`
//! - `2024-11-05`
//!
//! ### トランスポート
//!
//! - stdio
//!
//! SSE は未対応です。ただし、トランスポートは拡張可能なためカスタムトランスポートを実装することは可能です。
//!
//! ### メソッド
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
//! この crate による MCP サーバは非同期ランタイム tokio 上で動作します。
//!
//! `#[tokio::main]` で非同期ランタイムを起動し [`serve_stdio`] 関数に [`McpServer`] トレイトを実装した値を渡すことで
//! 標準入出力をトランスポートとするサーバが起動します。
//!
//! `McpServer` トレイトは手動で実装することもできますが、[`#[mcp_server]`](https://docs.rs/mcp-attr/latest/mcp_attr/server/attr.mcp_server.html) 属性を付与することで宣言的な方法で効率的に実装できます。
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
//!   #[tool]
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
//! また、`description` 属性パラメータを使用して説明を指定することもできます。ドキュメントコメントと description 属性の両方が指定されている場合、description 属性が優先されます。
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
//! ### クライアント機能の呼び出し
//!
//! MCP サーバは [`RequestContext`] を使用してクライアント機能([`roots/list`]など)を呼び出すことができます。
//!
//! 属性を使用して実装されたメソッドで `ResuestContext` を使用するには、メソッドの引数に `&ResuestContext` 型の変数を追加します。
//!
//! ```rust
//! use mcp_attr::server::{mcp_server, McpServer, RequestContext};
//! use mcp_attr::Result;
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   #[prompt]
//!   async fn echo_roots(&self, context: &RequestContext) -> Result<String> {
//!     let roots = context.roots_list().await?;
//!     Ok(format!("{:?}", roots))
//!   }
//! }
//! ```
//!
//! ### ドキュメントコメントからのinstructions
//!
//! `impl McpServer` ブロックのドキュメントコメントから [`instructions`] メソッドが自動生成されます。サーバーについて説明するドキュメントコメントを書くと、それがMCPクライアントにinstructionsとして送信されます。
//!
//! ```rust
//! use mcp_attr::server::{mcp_server, McpServer};
//! use mcp_attr::Result;
//!
//! struct ExampleServer;
//!
//! /// このサーバーはファイル操作とユーティリティを提供します。
//! /// 様々なファイル形式を扱い、データ変換を実行できます。
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!     #[tool]
//!     async fn hello(&self) -> Result<String> {
//!         Ok("Hello, world!".to_string())
//!     }
//! }
//! ```
//!
//! [`instructions`] メソッドが手動実装されている場合は手動実装が使用され、ドキュメントコメントからのinstructions生成は行われません。
//!
//! ### 補完機能サポート (`#[complete]`)
//!
//! `#[complete(function)]` 属性を使用してプロンプトやリソースの引数に補完機能を追加できます。
//!
//! 補完関数は以下のシグネチャを持つ必要があります：
//! - メソッド形式（`.method_name`）：`async fn func_name(&self, p: &CompleteRequestParams, cx: &RequestContext) -> Result<CompleteResult>`
//! - グローバル関数形式（`method_name`）：`async fn func_name(p: &CompleteRequestParams, cx: &RequestContext) -> Result<CompleteResult>`
//!
//! `#[complete]` 属性が使用されている場合、`completion_complete` メソッドが自動生成されます。手動実装がある場合は自動生成をスキップします。
//!
//! 補完関数の開発を容易にするため、`#[complete_fn]` 属性を使用して簡潔なシグネチャから必要なシグネチャへの自動変換ができます：
//!
//! ```rust
//! use mcp_attr::server::{mcp_server, McpServer, RequestContext, complete_fn};
//! use mcp_attr::Result;
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!     #[prompt]
//!     async fn greet(&self, #[complete(.complete_names)] name: String) -> Result<String> {
//!         Ok(format!("Hello, {name}!"))
//!     }
//!
//!     #[resource("files://{path}")]
//!     async fn get_file(&self, #[complete(.complete_paths)] path: String) -> Result<String> {
//!         Ok(format!("File: {path}"))
//!     }
//!
//!     // #[complete_fn]を#[mcp_server]ブロック内に記述
//!     #[complete_fn]
//!     async fn complete_paths(&self, _value: &str) -> Result<Vec<String>> {
//!         Ok(vec!["home".to_string(), "usr".to_string()])
//!     }
//!
//!     // RequestContextが必要な場合
//!     #[complete_fn]
//!     async fn complete_names(&self, _value: &str, _cx: &RequestContext) -> Result<Vec<&'static str>> {
//!         Ok(vec!["Alice", "Bob"])
//!     }
//! }
//! ```
//!
//! `#[complete_fn]` 属性では `cx: &RequestContext` パラメータを省略できます。RequestContextが不要な場合は省略することで、よりシンプルな補完関数を記述できます。
//!
//! 補完機能は `#[prompt]` と `#[resource]` の引数でのみ使用可能で、`#[tool]` の引数では使用できません。
//!
//! ## 各属性の説明
//!
//! ### `#[prompt]`
//!
//! ```rust,ignore
//! #[prompt("name", description = "..", title = "..")]
//! async fn func_name(&self) -> Result<GetPromptResult> { }
//! ```
//!
//! - "name" (optional) : プロンプト名。省略した場合は関数名が使用される。
//! - "description" (optional) : AI向けの関数説明。ドキュメントコメントより優先される。
//! - "title" (optional) : 人間が読みやすいプロンプトタイトル。
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
//! 引数には `#[complete(function)]` 属性を付与することで補完機能を追加できる。
//! 詳細は[補完機能サポート](#補完機能サポート-complete)を参照。
//!
//! 戻り値: [`Result<impl Into<GetPromptResult>>`]
//!
//! ```rust
//! use mcp_attr::Result;
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
//! #[resource("url_template", name = "..", mime_type = "..", description = "..", title = "..")]
//! async fn func_name(&self) -> Result<ReadResourceResult> { }
//! ```
//!
//! - "url_template" (optional) : このメソッドで処理するリソースの URL を示す URI Template([RFC 6570])。省略した場合は全ての URL を処理する。
//! - "name" (optional) : リソース名。省略した場合は関数名が使用される。
//! - "mime_type" (optional) : リソースの MIME タイプ。
//! - "description" (optional) : AI向けの関数説明。ドキュメントコメントより優先される。
//! - "title" (optional) : 人間が読みやすいリソースタイトル。
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
//! 引数には `#[complete(function)]` 属性を付与することで補完機能を追加できる。
//! 詳細は[補完機能サポート](#補完機能サポート-complete)を参照。
//!
//! URI Template は [RFC 6570] Level2 で指定。下記の 3 種類の変数が使用できる。
//!
//! - `{var}`
//! - `{+var}`
//! - `{#var}`
//!
//! 戻り値: [`Result<impl Into<ReadResourceResult>>`]
//!
//! ```rust
//! use mcp_attr::Result;
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
//! #[tool(
//!     "name",
//!     description = "..",
//!     title = "..",
//!     non_destructive,
//!     idempotent,
//!     read_only,
//!     closed_world,
//! )]
//! async fn func_name(&self) -> Result<CallToolResult> { }
//! ```
//!
//! - "name" (optional) : ツール名。省略した場合は関数名が使用される。
//! - "description" (optional) : AI向けの関数説明。ドキュメントコメントより優先される。
//! - "title" (optional) : 人間が読みやすいツールタイトル。
//! - "non_destructive" (optional) : ツールが追加的な更新のみを実行する (MCP仕様: `destructive = false`)
//! - "idempotent" (optional) : 同じ引数でツールを繰り返し呼び出しても追加の効果がない (MCP仕様: `idempotent = true`)
//! - "read_only" (optional) : ツールが環境を変更しない (MCP仕様: `read_only = true`)
//! - "closed_world" (optional) : ツールの相互作用ドメインが閉じている (MCP仕様: `open_world = false`)
//!
//! 下記のメソッドを実装する。
//!
//! - [`tools_list`]
//! - [`tools_call`]
//!
//! 関数の引数はツールの引数となる。引数は下記のすべてのトレイトの実装が必要。
//!
//! - [`DeserializeOwned`]: JSON から値を復元する為のトレイト
//! - [`JsonSchema`]: JSON Schema を生成する為のトレイト（JSON Schema は MCP Client に送信され、AI が引数の構造を理解できるようになる）
//!
//! 引数には `#[arg("name")]` 属性を付与することで名前を指定できる。
//! 指定しない場合は関数引数名の最初から `_` が取り除かれた名前が使用される。
//!
//! 戻り値: [`Result<impl Into<CallToolResult>>`]
//!
//! ```rust
//! use mcp_attr::Result;
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
//! 下記のメソッドは属性による実装に対応しておらず、手動での実装のみが可能です：
//!
//! - [`server_info`]
//!
//! 次のメソッドは、属性による実装を手動での実装で上書きすることができます：
//!
//! - [`completion_complete`] (`#[complete]` 属性使用時に自動生成)
//! - [`resources_list`]
//! - [`instructions`]
//!
//! ## テスト方法
//!
//! AI Coding Agent の登場によりテストはより重要になりました。
//! AI はテスト無しでは正しいコードをほとんど書く事ができませんが、テストがあればテストと修正を繰り返すことで正しいコードを書くことができるでしょう。
//!
//! mcp_attr にはプロセス内で MCP サーバと接続するテスト用の [`McpClient`] が含まれています。
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
//!     let client = McpClient::with_server(ExampleServer).await?;
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
//! [Model Context Protocol]: https://modelcontextprotocol.io/specification/2025-06-18/
//! [RFC 6570]: https://www.rfc-editor.org/rfc/rfc6570.html
//! [`prompts/list`]: https://modelcontextprotocol.io/specification/2025-06-18/server/prompts#listing-prompts
//! [`prompts/get`]: https://modelcontextprotocol.io/specification/2025-06-18/server/prompts#getting-a-prompt
//! [`resources/list`]: https://modelcontextprotocol.io/specification/2025-06-18/server/resources#listing-resources
//! [`resources/read`]: https://modelcontextprotocol.io/specification/2025-06-18/server/resources#reading-resources
//! [`resources/templates/list`]: https://modelcontextprotocol.io/specification/2025-06-18/server/resources#resource-templates
//! [`tools/list`]: https://modelcontextprotocol.io/specification/2025-06-18/server/tools#listing-tools
//! [`tools/call`]: https://modelcontextprotocol.io/specification/2025-06-18/server/tools#calling-tools
//! [`roots/list`]: https://modelcontextprotocol.io/specification/2025-06-18/client/roots#listing-roots
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
//! [`CompleteResult`]: https://docs.rs/mcp-attr/latest/mcp_attr/schema/struct.CompleteResult.html
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
//! [`Result<impl Into<GetPromptResult>>`]: https://docs.rs/mcp-attr/latest/mcp_attr/schema/struct.GetPromptResult.html
//! [`Result<impl Into<ReadResourceResult>>`]: https://docs.rs/mcp-attr/latest/mcp_attr/schema/struct.ReadResourceResult.html
//! [`Result<impl Into<CallToolResult>>`]: https://docs.rs/mcp-attr/latest/mcp_attr/schema/struct.CallToolResult.html
//! [`RequestContext`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/struct.RequestContext.html
//! [`serve_stdio`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/fn.serve_stdio.html
// #![include_doc("../../README.ja.md", end)]
// #![include_doc("../../README.md", start)]
//! # mcp-attr
//!
//! [![Crates.io](https://img.shields.io/crates/v/mcp-attr.svg)](https://crates.io/crates/mcp-attr)
//! [![Docs.rs](https://docs.rs/mcp-attr/badge.svg)](https://docs.rs/mcp-attr/)
//! [![Actions Status](https://github.com/frozenlib/mcp-attr/workflows/CI/badge.svg)](https://github.com/frozenlib/mcp-attr/actions)
//!
//! A library for declaratively building Model Context Protocol servers.
//!
//! ## Features
//!
//! mcp-attr is a crate designed to make it easy for both humans and AI to create [Model Context Protocol] servers.
//! To achieve this goal, it has the following features:
//!
//! - **Declarative Description**:
//!   - Use attributes like `#[mcp_server]` to describe MCP servers with minimal code
//!   - Fewer lines of code make it easier for humans to understand and consume less context window for AI
//! - **DRY (Don't Repeat Yourself) Principle**:
//!   - Declarative description ensures code follows the DRY principle
//!   - Prevents AI from writing inconsistent code
//! - **Leveraging the Type System**:
//!   - Expressing information sent to MCP clients through types reduces source code volume and improves readability
//!   - Type errors help AI with coding
//! - **`rustfmt` Friendly**:
//!   - Only uses attribute macros that can be formatted by `rustfmt`
//!   - Ensures AI-generated code can be reliably formatted
//!
//! ## Quick Start
//!
//! ### Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! mcp-attr = "0.0.7"
//! tokio = "1.43.0"
//! ```
//!
//! ### Example
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
//!     #[tool]
//!     async fn add_count(&self, message: String) -> Result<String> {
//!         let mut state = self.0.lock().unwrap();
//!         state.count += 1;
//!         Ok(format!("Echo: {message} {}", state.count))
//!     }
//!
//!     #[resource("my_app://files/{name}.txt")]
//!     async fn read_file(&self, name: String) -> Result<String> {
//!         Ok(format!("Content of {name}.txt"))
//!     }
//!
//!     #[prompt]
//!     async fn example_prompt(&self) -> Result<&str> {
//!         Ok("Hello!")
//!     }
//! }
//! ```
//!
//! ## Support Status
//!
//! ### Protocol Versions
//!
//! - `2025-03-26`
//! - `2024-11-05`
//!
//! ### Transport
//!
//! - stdio
//!
//! SSE is not yet supported. However, transport is extensible, so custom transports can be implemented.
//!
//! ### Methods
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
//! Start the server by launching the async runtime with `#[tokio::main]` and passing a value implementing the [`McpServer`] trait to the [`serve_stdio`] function,
//! which starts a server using standard input/output as transport.
//!
//! While you can implement the `McpServer` trait manually, you can implement it more efficiently in a declarative way by using the [`#[mcp_server]`](https://docs.rs/mcp-attr/latest/mcp_attr/server/attr.mcp_server.html) attribute.
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
//!   #[tool]
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
//! You can also specify descriptions using the `description` attribute parameter. When both documentation comments and description attributes are specified, the description attribute takes precedence.
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
//!             bail_public!(ErrorCode::INTERNAL_ERROR, "Error message");
//!         }
//!         if something_wrong {
//!             bail!("Error message");
//!         }
//!         let a = a.parse::<i32>()?;
//!         Ok(format!("Success {a}"))
//!     }
//! }
//! ```
//!
//! ### Calling Client Features
//!
//! MCP servers can call client features (such as [`roots/list`]) using [`RequestContext`].
//!
//! To use `RequestContext` in methods implemented using attributes, add a `&RequestContext` type variable to the method arguments.
//!
//! ```rust
//! use mcp_attr::server::{mcp_server, McpServer, RequestContext};
//! use mcp_attr::Result;
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!   #[prompt]
//!   async fn echo_roots(&self, context: &RequestContext) -> Result<String> {
//!     let roots = context.roots_list().await?;
//!     Ok(format!("{:?}", roots))
//!   }
//! }
//! ```
//!
//! ### Instructions from Documentation Comments
//!
//! The [`instructions`] method is automatically generated from documentation comments on the `impl McpServer` block. If you write documentation comments describing your server, they will be sent to the MCP client as instructions.
//!
//! ```rust
//! use mcp_attr::server::{mcp_server, McpServer};
//! use mcp_attr::Result;
//!
//! struct ExampleServer;
//!
//! /// This server provides file operations and utilities.
//! /// It can handle various file formats and perform data transformations.
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!     #[tool]
//!     async fn hello(&self) -> Result<String> {
//!         Ok("Hello, world!".to_string())
//!     }
//! }
//! ```
//!
//! If the [`instructions`] method is manually implemented, the manual implementation is used and automatic instructions generation from documentation comments is not performed.
//!
//! ### Completion Support (`#[complete]`)
//!
//! You can add completion functionality to prompt and resource arguments using the `#[complete(function)]` attribute.
//!
//! Completion functions must have the signature:
//! - Method form (`.method_name`): `async fn func_name(&self, p: &CompleteRequestParams, cx: &RequestContext) -> Result<CompleteResult>`
//! - Global function form (`method_name`): `async fn func_name(p: &CompleteRequestParams, cx: &RequestContext) -> Result<CompleteResult>`
//!
//! When `#[complete]` attributes are used, the `completion_complete` method is automatically generated. Manual implementation overrides auto-generation.
//!
//! To make completion function development easier, you can use the `#[complete_fn]` attribute to automatically convert simple signatures to the required signature:
//!
//! ```rust
//! use mcp_attr::server::{mcp_server, McpServer, RequestContext, complete_fn};
//! use mcp_attr::Result;
//!
//! struct ExampleServer;
//!
//! #[mcp_server]
//! impl McpServer for ExampleServer {
//!     #[prompt]
//!     async fn greet(&self, #[complete(.complete_names)] name: String) -> Result<String> {
//!         Ok(format!("Hello, {name}!"))
//!     }
//!
//!     #[resource("files://{path}")]
//!     async fn get_file(&self, #[complete(.complete_paths)] path: String) -> Result<String> {
//!         Ok(format!("File: {path}"))
//!     }
//!
//!     // #[complete_fn] can be written inside #[mcp_server] block
//!     #[complete_fn]
//!     async fn complete_paths(&self, _value: &str) -> Result<Vec<String>> {
//!         Ok(vec!["home".to_string(), "usr".to_string()])
//!     }
//!
//!     // When RequestContext is needed
//!     #[complete_fn]
//!     async fn complete_names(&self, _value: &str, _cx: &RequestContext) -> Result<Vec<&'static str>> {
//!         Ok(vec!["Alice", "Bob"])
//!     }
//! }
//! ```
//!
//! The `#[complete_fn]` attribute allows omitting the `cx: &RequestContext` parameter. When RequestContext is not needed, you can omit it for simpler completion functions.
//!
//! Completion is only available for `#[prompt]` and `#[resource]` arguments, not for `#[tool]` arguments.
//!
//! ## Attribute Descriptions
//!
//! ### `#[prompt]`
//!
//! ```rust,ignore
//! #[prompt("name", description = "..", title = "..")]
//! async fn func_name(&self) -> Result<GetPromptResult> { }
//! ```
//!
//! - "name" (optional): Prompt name. If omitted, the function name is used.
//! - "description" (optional): Function description for AI. Takes precedence over documentation comments.
//! - "title" (optional): Human-readable prompt title.
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
//! Arguments can have completion functionality using the `#[complete(function)]` attribute.
//! See [Completion Support](#completion-support-complete) for details.
//!
//! Return value: [`Result<impl Into<GetPromptResult>>`]
//!
//! ```rust
//! use mcp_attr::Result;
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
//! #[resource("url_template", name = "..", mime_type = "..", description = "..", title = "..")]
//! async fn func_name(&self) -> Result<ReadResourceResult> { }
//! ```
//!
//! - "url_template" (optional): URI Template ([RFC 6570]) indicating the URL of resources this method handles. If omitted, handles all URLs.
//! - "name" (optional): Resource name. If omitted, the function name is used.
//! - "mime_type" (optional): MIME type of the resource.
//! - "description" (optional): Function description for AI. Takes precedence over documentation comments.
//! - "title" (optional): Human-readable resource title.
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
//! Arguments can have completion functionality using the `#[complete(function)]` attribute.
//! See [Completion Support](#completion-support-complete) for details.
//!
//! URI Templates are specified in [RFC 6570] Level2. The following variables can be used in URI Templates:
//!
//! - `{var}`
//! - `{+var}`
//! - `{#var}`
//!
//! Return value: [`Result<impl Into<ReadResourceResult>>`]
//!
//! ```rust
//! use mcp_attr::Result;
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
//! #[tool(
//!     "name",
//!     description = "..",
//!     title = "..",
//!     non_destructive,
//!     idempotent,
//!     read_only,
//!     closed_world,
//! )]
//! async fn func_name(&self) -> Result<CallToolResult> { }
//! ```
//!
//! - "name" (optional): Tool name. If omitted, the function name is used.
//! - "description" (optional): Function description for AI. Takes precedence over documentation comments.
//! - "title" (optional): Human-readable tool title.
//! - "non_destructive" (optional): Tool performs only additive updates (MCP spec: `destructive = false`)
//! - "idempotent" (optional): Calling the tool repeatedly with the same arguments has no additional effect (MCP spec: `idempotent = true`)
//! - "read_only" (optional): Tool does not modify its environment (MCP spec: `read_only = true`)
//! - "closed_world" (optional): Tool's domain of interaction is closed (MCP spec: `open_world = false`)
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
//! Return value: [`Result<impl Into<CallToolResult>>`]
//!
//! ```rust
//! use mcp_attr::Result;
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
//! The following methods do not support implementation through attributes and must be implemented manually:
//!
//! - [`server_info`]
//!
//! The following methods can be overridden with manual implementation over the attribute-based implementation:
//!
//! - [`completion_complete`] (auto-generated when `#[complete]` attributes are used)
//! - [`resources_list`]
//! - [`instructions`]
//!
//! ## Testing
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
//!     let client = McpClient::with_server(ExampleServer).await?;
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
//! [Model Context Protocol]: https://modelcontextprotocol.io/specification/2025-06-18/
//! [RFC 6570]: https://www.rfc-editor.org/rfc/rfc6570.html
//! [`prompts/list`]: https://modelcontextprotocol.io/specification/2025-06-18/server/prompts#listing-prompts
//! [`prompts/get`]: https://modelcontextprotocol.io/specification/2025-06-18/server/prompts#getting-a-prompt
//! [`resources/list`]: https://modelcontextprotocol.io/specification/2025-06-18/server/resources#listing-resources
//! [`resources/read`]: https://modelcontextprotocol.io/specification/2025-06-18/server/resources#reading-resources
//! [`resources/templates/list`]: https://modelcontextprotocol.io/specification/2025-06-18/server/resources#resource-templates
//! [`tools/list`]: https://modelcontextprotocol.io/specification/2025-06-18/server/tools#listing-tools
//! [`tools/call`]: https://modelcontextprotocol.io/specification/2025-06-18/server/tools#calling-tools
//! [`roots/list`]: https://modelcontextprotocol.io/specification/2025-06-18/client/roots#listing-roots
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
//! [`CompleteResult`]: https://docs.rs/mcp-attr/latest/mcp_attr/schema/struct.CompleteResult.html
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
//! [`Result<impl Into<GetPromptResult>>`]: https://docs.rs/mcp-attr/latest/mcp_attr/schema/struct.GetPromptResult.html
//! [`Result<impl Into<ReadResourceResult>>`]: https://docs.rs/mcp-attr/latest/mcp_attr/schema/struct.ReadResourceResult.html
//! [`Result<impl Into<CallToolResult>>`]: https://docs.rs/mcp-attr/latest/mcp_attr/schema/struct.CallToolResult.html
//! [`RequestContext`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/struct.RequestContext.html
//! [`serve_stdio`]: https://docs.rs/mcp-attr/latest/mcp_attr/server/fn.serve_stdio.html
// #![include_doc("../../README.md", end)]
