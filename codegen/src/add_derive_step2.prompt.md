1. `cargo run -p mcp-attr-codegen2` を実行し、このプロジェクトに適用します。
2. `cargo build` を実行し、エラーが無いかを確認します。
3. エラーが発生した場合は`derive_default.rs` の `DEFAULT_TYPE_NAMES` に書かれた型名を適切に追加、削除し、手順 1 に戻ります。`DEFAULT_TYPE_NAMES` の型一覧変更ではエラーを修正できないと判断した場合は、その理由をユーザーに説明し停止します。
4. エラーが発生しなくなったら完了です。
