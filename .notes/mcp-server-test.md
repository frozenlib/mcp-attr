# McpServer のテスト方針

このドキュメントでは、`#[mcp_server]`属性を使用したサーバー実装のテスト方針について説明します。
ユーザーから他のテストでも利用可能な汎用的な方針が示された場合はこのファイルの内容も更新してください。

## 基本テスト構造

テストは`./tests/<file_name>.rs`に記述します。テストファイルの基本構造は以下の通りです：

```rust
use std::collections::HashMap;

use mcp_attr::{jsoncall, mcp_server, McpServer};
use mcp_attr::schema::{GetPromptRequestParams, GetPromptResult};
use mcp_attr::Result;
use serde_json::Value as JsonValue;

struct MyMcpServer;

#[mcp_server]
impl McpServer for MyMcpServer {
    // サーバーメソッドの実装
}

// テスト関数
#[tokio::test]
async fn test_method_name() -> Result<()> {
    // テスト内容
    Ok(())
}
```

## 正常系テスト

正常系のテストでは、以下の点を確認します：

1. サーバーメソッドが正しく呼び出されること
2. 戻り値が期待通りであること

```rust
#[tokio::test]
async fn test_normal_case() -> Result<()> {
    let server = MyMcpServer::new();
    let result = server.get_prompt("method_name", &[arg1, arg2]).await?;

    // 期待される結果と比較
    assert_eq!(result, expected_result);

    Ok(())
}
```

## エラー系テスト

エラー系のテストでは、以下の点を確認します：

1. エラーが発生すること
2. エラーコードが期待通りであること
3. エラーメッセージが期待通りであること

```rust
#[tokio::test]
async fn test_error_case() -> Result<()> {
    let server = MyMcpServer::new();
    let result = server.get_prompt("method_name", &[/* 不正な引数 */]).await;

    // エラーが返されることを確認
    assert!(result.is_err());

    // エラーの詳細を確認
    if let Err(err) = result {
        println!("Error type: {}", std::any::type_name_of_val(&err));
        println!("Error: {}", err);

        // エラーオブジェクトを取得
        if let Some(error_object) = err.error_object() {
            // エラーコードを確認
            assert_eq!(error_object.code.0, mcp_attr::jsoncall::ErrorCode::EXPECTED_ERROR_CODE.0);

            // エラーメッセージを確認
            println!("Error message: {}", error_object.message);
            assert!(error_object.message.contains("expected message part"));
        } else {
            panic!("Expected error object");
        }
    }

    Ok(())
}
```

## エラーコードの種類

主に使用されるエラーコードは以下の通りです：

- `jsoncall::ErrorCode::INVALID_PARAMS` (-32602) - 引数が不正な場合やプロンプトが見つからない場合
- `jsoncall::ErrorCode::INTERNAL_ERROR` (-32603) - サーバー内部でエラーが発生した場合

## テスト対象

以下のようなケースを必ずテストするようにしてください：

1. 正常系のテスト

   - 正しい引数でメソッドを呼び出し、正しい結果が返ることを確認

2. 引数不足のテスト

   - 必要な引数が渡されなかった場合のエラー処理を確認
   - エラーコードが `INVALID_PARAMS` であることを確認

3. 存在しないメソッド名のテスト

   - 存在しないメソッド名が指定された場合のエラー処理を確認
   - エラーコードが `INVALID_PARAMS` であることを確認

4. メソッド固有のエラー条件のテスト
   - メソッド内部でエラーが発生する条件を設定し、エラーが正しく処理されることを確認
   - エラーコードが期待通りであることを確認（多くの場合 `INTERNAL_ERROR`）

## デバッグ方法

テストが失敗した場合は、以下の情報を追加して原因を特定しやすくしてください：

1. セッションエラーの詳細な情報を表示

   ```rust
   println!("Error type: {}", std::any::type_name_of_val(&err));
   println!("Error: {}", err);
   ```

2. エラーオブジェクトが存在する場合はそのメッセージを表示

   ```rust
   if let Some(error_object) = err.error_object() {
      println!("Error message: {}", error_object.message);
   }
   ```

これらの情報により、エラーの原因特定が容易になります。
