# `#[derive(Default)]` を適用する型一覧を計算する

次のコードのを元に `codegen2/src/derive_default.rs`に関数 `derive_default_types` を実装してください。

`tests` モジュール(ファイルは `codegen2/src/derive_default/tests.rs` )にテストを作成し、テストをパスするまでコードを修正してください。

```rust
use syn::{Ident, Type};

#[cfg(test)]
mod tests;

/// 外部の型について `Default` が実装されているかどうかを判定する
/// derive_default_typesに入力するファイルで定義された型についてはfalseを返す
fn is_default_types(ty: &Type) -> bool {
    if type_of("String") == *ty {
        return true;
    }
    // todo: Vec<T>やOption<T>などについてtrueを返すようにすべき
    todo!()
}

/// file定義された型のうち、`#[derive(Default)]` を指定可能な型を返す
///
/// それは全てのフィールドが次のいずれかの条件を満たす構造体である
///
/// - is_default_typesがtrueを返す型
/// - derive_default_typesの結果に含まれる型
pub fn derive_default_types(file: &mut syn::File) -> Result<Vec<Ident>> {
    todo!()
}

fn type_of(s: &str) -> Type {
    syn::parse_str::<Type>(s).unwrap()
}
```
