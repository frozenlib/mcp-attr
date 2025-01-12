次のコードのを元に `codegen2/src/add_derive.rs`に関数 `add_derive` を実装してください。

`tests` モジュール(ファイルは `codegen2/src/add_derive/tests.rs` )にテストを作成し、テストをパスするまでコードを修正してください。

テストは `cargo test -p mcp-attrs-codegen2 --lib -- add_derive` で実行します。

```rust
use syn::Ident;

#[cfg(test)]
mod tests;

/// fileで定義された型のうち、typesで指定されたものについて `#[derive(Trait0,Trait1)]` のようなderiveを追加します。
/// deriveの中身は引数traitsで指定します。
pub fn add_derive(file: &mut syn::File, types: &[Ident], traits: &[&str]) -> syn::Result<()> {
}
```
