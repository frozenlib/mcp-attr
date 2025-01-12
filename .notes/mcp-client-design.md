# MPC Client の実装に関して

`mod client` に含まれる MPC Client の実装に関して
（`mod client`の内容は現在未実装です）

## 目的

1. `mod server` で実装された MPC Server セットで使用することで、`mod server`のテストを行えるようにする(最も重要)
2. AI Agent の実装の為に使用できる使いやすい MPC Client 実装を提供する

## 詳細

### 全体設計

MPC クライアントは以下の機能を提供します：

1. MPC プロトコルに準拠したクライアント実装
2. サーバーとの通信を容易にするためのインターフェース
3. テスト用の機能（`mod server`のテストを容易にするため）
4. AI Agent の実装を支援するためのヘルパー機能

### 構造

```rust
pub struct McpClientSession {
    // 内部状態
    jsoncall_session: Arc<dyn jsoncall::Session>,
    capabilities: Option<ServerCapabilities>,
}

pub struct McpClientSessionBuilder {
    // クライアント構築のためのオプション
    protocol_version: String,
    client_capabilities: ClientCapabilities,
    jsoncall_session: Option<Arc<dyn jsoncall::Session>>,
}

pub struct McpClient {
    session: Arc<McpClientSession>,
    tools: Vec<Tool>,
    prompts: Vec<String>,
    resources: HashMap<String, Resource>,
}

pub struct McpClientBuilder {
    // McpClient構築のためのオプション
    protocol_version: String,
    client_capabilities: ClientCapabilities,
    jsoncall_session: Option<Arc<dyn jsoncall::Session>>,
    // 追加設定オプション
    auto_refresh: bool,
}

pub struct RequestContext {
    // リクエスト固有のコンテキスト
    session: Arc<McpClientSession>,
    request_id: RequestId,
}
```

### インターフェース

#### 基本的な通信インターフェース

```rust
impl McpClientSessionBuilder {
    pub fn new() -> Self;
    pub fn protocol_version(self, version: impl Into<String>) -> Self;
    pub fn client_capabilities(self, capabilities: ClientCapabilities) -> Self;
    pub fn session(self, jsoncall_session: Arc<dyn jsoncall::Session>) -> Self;
    pub async fn build(self) -> Result<McpClientSession>;
    // build メソッドは内部で initialize/initialized リクエストを自動的に実行し、
    // 初期化が完了した状態の McpClientSession を返します。
    // このため、ユーザーは個別に初期化処理を行う必要はありません。
}

impl McpClientSession {
    // ビルダーの取得
    pub fn builder() -> McpClientSessionBuilder;

    // サーバー情報
    pub fn server_capabilities(&self) -> Option<&ServerCapabilities>;

    // 以下のメソッドは、サーバーの対応するAPIへのアクセスを提供
    pub async fn prompts_list(&self, params: ListPromptsRequestParams) -> Result<ListPromptsResult>;
    pub async fn prompts_get(&self, params: GetPromptRequestParams) -> Result<GetPromptResult>;
    pub async fn resources_list(&self, params: ListResourcesRequestParams) -> Result<ListResourcesResult>;
    pub async fn resources_templates_list(&self, params: ListResourceTemplatesRequestParams) -> Result<ListResourceTemplatesResult>;
    pub async fn resources_read(&self, params: ReadResourceRequestParams) -> Result<ReadResourceResult>;
    pub async fn tools_list(&self, params: ListToolsRequestParams) -> Result<ListToolsResult>;
    pub async fn tools_call(&self, params: CallToolRequestParams) -> Result<CallToolResult>;
    pub async fn ping(&self) -> Result<()>;
}
```

#### AI Agent 用インターフェース

```rust
impl McpClientBuilder {
    pub fn new() -> Self;
    pub fn protocol_version(self, version: impl Into<String>) -> Self;
    pub fn client_capabilities(self, capabilities: ClientCapabilities) -> Self;
    pub fn session(self, jsoncall_session: Arc<dyn jsoncall::Session>) -> Self;
    pub fn auto_refresh(self, auto_refresh: bool) -> Self;
    pub async fn build(self) -> Result<McpClient>;
    // build メソッドは内部で McpClientSession を作成し初期化します。
    // auto_refreshがtrueの場合、ツール、プロンプト、リソースの
    // 初期取得も自動的に行います。
}

impl McpClient {
    // ビルダーの取得
    pub fn builder() -> McpClientBuilder;

    // 直接インスタンス化（既存のMcpClientSessionから）
    pub fn new(session: Arc<McpClientSession>) -> Self;

    // 内部のMcpClientSessionへのアクセス
    pub fn session(&self) -> &Arc<McpClientSession>;

    // キャッシュを更新するための便利なメソッド
    pub async fn refresh_tools(&mut self) -> Result<()>;
    pub async fn refresh_prompts(&mut self) -> Result<()>;
    pub async fn refresh_resources(&mut self) -> Result<()>;
    pub async fn refresh_url_templates(&mut self) -> Result<()>;
    pub async fn refresh_all(&mut self) -> Result<()>;

    // リソースへのアクセス - シンプルな命名
    pub fn tool(&self, name: &str) -> Option<&Tool>;
    pub fn tools(&self) -> &[Tool];

    pub fn prompt(&self, name: &str) -> Option<&Prompt>;
    pub fn prompts(&self) -> &[Prompt];

    // プロンプト内容を直接サーバーから取得（prompts/get APIに対応）
    pub async fn get_prompt<T: Serialize>(&self, name: &str, args: Option<T>) -> Result<PromptContent>;

    pub fn resource(&self, uri: &str) -> Option<&Resource>;
    pub fn resources(&self) -> &HashMap<String, Resource>;

    // リソース内容を直接サーバーから取得（resources/read APIに対応）
    pub async fn read_resource(&self, uri: &str) -> Result<ResourceContent>;

    pub fn url_template(&self, name: &str) -> Option<&UrlTemplate>;
    pub fn url_templates(&self) -> &HashMap<String, UrlTemplate>;

    // ツールの呼び出し - シンプル化
    pub async fn call_tool<T: Serialize>(&self, name: &str, args: T) -> Result<CallToolResult>;
}
```

### 実装方針

1. **標準通信チャネル**: `McpClientSession`は`jsoncall::Session`トレイトを実装する任意のセッションを使用できます。これにより、WebSocket、stdio、テスト用モックなど様々な通信チャネルに対応できます。

2. **適切なエラー処理**: すべての非同期操作は明示的なエラーハンドリングを提供し、エラー情報を適切に伝播します。

3. **モジュール性**: クライアント機能は小さなコンポーネントに分割し、テストと拡張を容易にします。

4. **テスト容易性**: モックを使用して、実際のサーバーとの接続なしにクライアント機能をテストできます。

5. **非同期対応**: すべての通信操作は非同期で実行され、現代的な非同期 Rust の原則に従います。

6. **タイプセーフ**: クライアント-サーバー通信で使用されるすべての型は、スキーマ定義から生成されたタイプセーフな構造体を使用します。

7. **自動初期化**: クライアントの作成時（McpClientSessionBuilder::build メソッド内）に自動的に初期化（initialize/initialized）プロセスを完了させます。build メソッドが返す時点では初期化処理が完了しており、返された McpClientSession インスタンスはすぐに使用可能な状態です。ユーザーはこれらの低レベル詳細を気にする必要がありません。

8. **柔軟なビルドパターン**: 用途に応じて、`McpClientSessionBuilder`か`McpClientBuilder`のいずれかを使用できます。低レベルの制御が必要な場合は前者を、高レベルの AI エージェント機能が必要な場合は後者を使用します。

9. **シンプルで統一された引数指定**: AI エージェント向けインターフェイスでは、汎用的な`Serialize`トレイトを活用して、様々な形式（JSON 値、構造体、マップなど）からツール引数を作成できます。これにより API の一貫性が保たれ、AI エージェントからの多様な出力形式に対応できます。

10. **名前付きリソースアクセス**: リソース、ツール、プロンプト、URL テンプレートなどのサーバーリソースに、シンプルで一貫性のある命名規則でアクセスできます。また、URL テンプレートを名前で参照し、必要に応じてテンプレートを展開できるようになっています。

### 使用例

#### 基本的な使用方法（McpClientBuilder を使用）

```rust
// JSonCallセッションを作成（stdio、WebSocketなど）
let jsoncall_session = create_stdio_session().await?;

// McpClientBuilderを使用して直接McpClientを作成
// （内部でMcpClientSessionを初期化し、必要なリソースも自動で取得）
let client = McpClient::builder()
    .session(Arc::new(jsoncall_session))
    .auto_refresh(true)  // ツール、プロンプト、リソースを自動取得
    .build()
    .await?;

// 直接ツールを呼び出す
let result = client.call_tool("sample_tool", json!({ "x": 1, "y": 2 })).await?;
println!("計算結果: {:?}", result);

// プロンプトを取得（キャッシュされたものがある場合）
if let Some(prompt) = client.prompt("system") {
    println!("システムプロンプト: {}", prompt.content);
}

// プロンプト内容を直接サーバーから取得
let system_prompt_content = client.get_prompt("system", None::<()>).await?;
println!("システムプロンプト: {}", system_prompt_content.content);

// 引数付きでプロンプトを取得
let user_prompt = client.get_prompt("user", json!({"user_name": "田中"})).await?;
println!("ユーザープロンプト: {}", user_prompt.content);

// ツール情報へのアクセス
if let Some(tool) = client.tool("sample_tool") {
    println!("ツール説明: {}", tool.description);
}

// リソースを直接サーバーから読み取る
let document_content = client.read_resource("documents/user_manual").await?;
println!("ドキュメント内容: {}", document_content.content);
```

#### McpClientSessionBuilder を使用する方法

```rust
// JSonCallセッションを作成
let jsoncall_session = create_stdio_session().await?;

// McpClientSessionを作成
let client_session = McpClientSession::builder()
    .session(Arc::new(jsoncall_session))
    .build()
    .await?;

// McpClientSessionを用いてMcpClientを作成
let mut client = McpClient::new(Arc::new(client_session));

// 必要なリソースを取得
client.refresh_all().await?;  // または個別に refresh_tools(), refresh_prompts() など

// ツールを呼び出す
let result = client.call_tool("sample_tool", json!({ "x": 1, "y": 2 })).await?;
```

#### セッションクライアントを直接使用

```rust
// セッションクライアントを作成
let jsoncall_session = create_stdio_session().await?;
let client_session = McpClientSession::builder()
    .session(Arc::new(jsoncall_session))
    .build()
    .await?;

// APIを直接呼び出す
let tools = client_session.tools_list(ListToolsRequestParams::default()).await?;

// ツールを直接呼び出す
let result = client_session.tools_call(CallToolRequestParams {
    name: "sample_tool".to_string(),
    arguments: serde_json::Map::new(),
}).await?;
```

#### McpClientBuilder でのカスタマイズした初期化

```rust
// カスタムのプロトコルバージョンとクライアント機能を設定
let jsoncall_session = create_stdio_session().await?;
let client = McpClient::builder()
    .session(Arc::new(jsoncall_session))
    .protocol_version("2.0")
    .client_capabilities(custom_capabilities)
    .auto_refresh(false)  // リソース自動取得を無効化
    .build()
    .await?;

// 特定のリソースのみ取得
client.refresh_tools().await?;

// ツールを呼び出す
let result = client.call_tool("calculate", json!({ "x": 1, "y": 2 })).await?;
```

#### McpClientSessionBuilder でのカスタマイズした初期化

```rust
// カスタムのプロトコルバージョンとクライアント機能を設定
let jsoncall_session = create_stdio_session().await?;
let client_session = McpClientSession::builder()
    .session(Arc::new(jsoncall_session))
    .protocol_version("2.0")
    .client_capabilities(custom_capabilities)
    .build()
    .await?;

// McpClientを作成
let client = McpClient::new(Arc::new(client_session));
```

#### AI エージェント用途

```rust
// JSonCallセッションを作成
let jsoncall_session = create_stdio_session().await?;

// McpClientBuilderを使用して直接McpClientを作成
let mut client = McpClient::builder()
    .session(Arc::new(jsoncall_session))
    .auto_refresh(true)  // すべてのリソースを自動取得
    .build()
    .await?;

// 内部のセッションにもアクセス可能
let session = client.session();

// 様々な方法でツールを呼び出す

// 1. インラインJSONの場合
let result1 = client.call_tool("calculate", json!({ "x": 1, "y": 2 })).await?;

// 2. 構造体を使用する場合
#[derive(Serialize)]
struct CalcArgs {
    x: i32,
    y: i32,
}
let args = CalcArgs { x: 1, y: 2 };
let result2 = client.call_tool("calculate", args).await?;

// 3. HashMapやVecを使用する場合
let mut map = HashMap::new();
map.insert("operation", "multiply");
map.insert("values", "[1, 2, 3, 4]");
let result3 = client.call_tool("calculate", map).await?;

// リソースアクセス
// ツールにアクセス
if let Some(tool) = client.tool("calculate") {
    println!("ツール情報: {:#?}", tool);
}

// すべてのツールを取得
let all_tools = client.tools();

// プロンプトにアクセス（キャッシュされたものがある場合）
if let Some(prompt) = client.prompt("system_prompt") {
    println!("プロンプト内容: {}", prompt.content);
    println!("プロンプト説明: {}", prompt.description);
}

// プロンプト内容を直接サーバーから取得
let system_prompt_content = client.get_prompt("system_prompt", None::<()>).await?;
println!("サーバーから取得したプロンプト内容: {}", system_prompt_content.content);

// 引数付きでプロンプトを取得
#[derive(Serialize)]
struct PromptArgs {
    user_name: String,
    role: String,
}
let args = PromptArgs {
    user_name: "田中".to_string(),
    role: "管理者".to_string()
};
let user_prompt = client.get_prompt("user_prompt", Some(args)).await?;
println!("カスタマイズされたプロンプト: {}", user_prompt.content);

// すべてのプロンプトを取得
let all_prompts = client.prompts();

// リソースにアクセス
if let Some(resource) = client.resource("data:example") {
    println!("キャッシュされたリソース内容: {}", resource.content);
}

// リソースを直接サーバーから読み取る
let latest_data = client.read_resource("data:example").await?;
println!("サーバーから最新取得したリソース内容: {}", latest_data.content);

// すべてのリソースを取得
let all_resources = client.resources();

// URLテンプレートにアクセス
if let Some(template) = client.url_template("user_profile") {
    // テンプレート情報の表示
    println!("URLテンプレート: {}", template.template);

    // URLの展開（自前で実装）
    let url = template.expand(&[("user_id", "abc123")]);
    println!("展開されたURL: {}", url);
}

// すべてのURLテンプレートを取得
let all_templates = client.url_templates();

// エージェントロジック実行
if let Some(calculate_tool) = client.tool("calculate") {
    let result = client.call_tool("calculate", json!({
        "operation": "multiply",
        "values": [1, 2, 3, 4]
    })).await?;
    println!("計算結果: {:?}", result);
}
```
