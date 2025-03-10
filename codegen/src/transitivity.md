# 型変換の推移的実装の生成

このモジュールは、型変換（`From` trait 実装）の推移的な関係を自動的に生成します。

## アルゴリズムの概要

1. 型定義の収集
2. 既存の型変換の収集
3. 推移的な型変換の生成

## データ構造

```rust
struct Registry {
    // 定義された型の集合
    defined_types: HashSet<Type>,
    // 既存の型変換（From実装）のリスト
    converts: Vec<Convert>,
    // 生成済みの型変換の集合（(変換元の型, 変換先の型)のタプル）
    generated_converts: HashSet<(Type, Type)>,
}
struct Convert {
    from: Type,
    to: Type,
}

```

## 詳細なステップ

### 1. 型定義の収集

1. 入力ファイルから構造体と enum の定義を探索
2. 定義された型を`defined_types`に格納

### 2. 既存の型変換の収集

1. 入力ファイルから`From`トレイトの実装を探索
2. 各実装から変換元の型と変換先の型を抽出し
3. 変換先がジェネリック型の場合は収集しない
4. 変換元の型と変換先の型を正規化
5. 抽出した型変換を`converts`に格納

### 3. 推移的な型変換の生成

1. 既存の型変換から変換関係のマップを構築

   - キー: 変換元の型
   - 値: 変換先の型のリスト

2. 以下のプロセスを新しい変換が見つからなくなるまで繰り返す：

   - 現在の変換マップをイテレート
   - A -> B, B -> C という変換が存在し、A -> C が未生成の場合：
     - 変換先の型（C）が`defined_types`に含まれているか確認
     - 含まれている場合、A -> C の変換を生成
     - 生成した変換を`generated_converts`に記録
     - 変換マップを更新

3. 生成した変換をソート

   - 変換元の型でソート
   - 変換元が同じ場合は変換先の型でソート

4. 各変換に対して`From`トレイトの実装を生成

   ```rust
   impl From<A> for C {
       fn from(value: A) -> Self {
           <C as ::std::convert::From<B>>::from(<B as ::std::convert::From<A>>::from(value))
       }
   }
   ```

## 型の正規化について

- `Self` キーワードは具体的な型名に変換する

## 制約と注意点

- 自己変換（A -> A）は生成しない
- 既に生成済みの変換は重複して生成しない

## テスト

どのようなテストが必要かを考え、テストを実行し、テストがパスするまで修正してください

- `cargo test -p mcp-attr-codegen2` でテストを実行する
- `impl From<&Self> for X` のような Self キーワードを含む場合のテストも行う
