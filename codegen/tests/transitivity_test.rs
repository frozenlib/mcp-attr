use anyhow::Result;
use std::fs;
use std::path::Path;

// テスト対象のモジュールをインポート
extern crate mcp_attr_codegen;
use mcp_attr_codegen::transitivity::build_transitivity;

#[test]
fn test_basic_transitivity() -> Result<()> {
    // テスト用のデータファイルのパス
    let test_data_path = Path::new("tests/test_data.rs");

    // テストデータファイルが存在することを確認
    assert!(
        test_data_path.exists(),
        "テストデータファイルが見つかりません"
    );

    // build_transitivityを実行
    let result = build_transitivity(&[test_data_path.to_str().unwrap()])?;

    // 生成されたコードを文字列に変換
    let generated_code = result.to_string();

    // デバッグ用に生成されたコードを出力
    println!("生成されたコード:\n{}", generated_code);

    // 期待される変換が含まれているか確認
    assert!(
        generated_code.contains("impl From < TypeA > for TypeC"),
        "TypeA -> TypeCの変換が生成されていません"
    );

    assert!(
        generated_code.contains("impl From < TypeA > for TypeD"),
        "TypeA -> TypeDの変換が生成されていません"
    );

    assert!(
        generated_code.contains("impl From < TypeB > for TypeD"),
        "TypeB -> TypeDの変換が生成されていません"
    );

    // 中間型を経由する実装になっているか確認
    assert!(
        generated_code.contains("let intermediate")
            && generated_code.contains("TypeB")
            && generated_code.contains("value . into"),
        "TypeA -> TypeCの変換で中間型TypeBが使用されていません"
    );

    assert!(
        generated_code.contains("let intermediate")
            && generated_code.contains("TypeC")
            && generated_code.contains("value . into"),
        "TypeB -> TypeDの変換で中間型TypeCが使用されていません"
    );

    Ok(())
}

#[test]
fn test_complex_transitivity() -> Result<()> {
    // 複雑なケース用のテストデータを一時ファイルに作成
    let complex_test_data = r#"
    pub struct ComplexA;
    pub struct ComplexB;
    pub struct ComplexC;
    pub struct ComplexD;
    pub struct ComplexE;
    
    // 基本的な変換の実装
    impl From<ComplexA> for ComplexB {
        fn from(_: ComplexA) -> Self {
            ComplexB
        }
    }
    
    impl From<ComplexB> for ComplexC {
        fn from(_: ComplexB) -> Self {
            ComplexC
        }
    }
    
    impl From<ComplexC> for ComplexD {
        fn from(_: ComplexC) -> Self {
            ComplexD
        }
    }
    
    impl From<ComplexD> for ComplexE {
        fn from(_: ComplexD) -> Self {
            ComplexE
        }
    }
    "#;

    let temp_file_path = "tests/complex_test_data.rs";
    fs::write(temp_file_path, complex_test_data)?;

    // build_transitivityを実行
    let result = build_transitivity(&[temp_file_path])?;

    // 生成されたコードを文字列に変換
    let generated_code = result.to_string();

    // 期待される変換が含まれているか確認
    assert!(
        generated_code.contains("impl From < ComplexA > for ComplexC"),
        "ComplexA -> ComplexCの変換が生成されていません"
    );

    assert!(
        generated_code.contains("impl From < ComplexA > for ComplexD"),
        "ComplexA -> ComplexDの変換が生成されていません"
    );

    assert!(
        generated_code.contains("impl From < ComplexA > for ComplexE"),
        "ComplexA -> ComplexEの変換が生成されていません"
    );

    assert!(
        generated_code.contains("impl From < ComplexB > for ComplexD"),
        "ComplexB -> ComplexDの変換が生成されていません"
    );

    assert!(
        generated_code.contains("impl From < ComplexB > for ComplexE"),
        "ComplexB -> ComplexEの変換が生成されていません"
    );

    assert!(
        generated_code.contains("impl From < ComplexC > for ComplexE"),
        "ComplexC -> ComplexEの変換が生成されていません"
    );

    // テスト後に一時ファイルを削除
    fs::remove_file(temp_file_path)?;

    Ok(())
}

#[test]
fn test_only_defined_types() -> Result<()> {
    // 外部型と内部型が混在するテストデータを一時ファイルに作成
    let mixed_test_data = r#"
    // 内部で定義された型
    pub struct InternalA;
    pub struct InternalB;
    
    // 外部型（仮想的に外部から来たものとして扱う）
    // 実際のテストでは、これらの型は定義されているが、
    // build_transitivityは「定義された型」としてこれらを認識しないはず
    pub struct ExternalC;
    pub struct ExternalD;
    
    // 基本的な変換の実装
    impl From<InternalA> for InternalB {
        fn from(_: InternalA) -> Self {
            InternalB
        }
    }
    
    impl From<InternalB> for ExternalC {
        fn from(_: InternalB) -> Self {
            ExternalC
        }
    }
    
    impl From<ExternalC> for ExternalD {
        fn from(_: ExternalC) -> Self {
            ExternalD
        }
    }
    "#;

    let temp_file_path = "tests/mixed_test_data.rs";
    fs::write(temp_file_path, mixed_test_data)?;

    // 別のファイルに外部型を定義（これらは「定義された型」として認識される）
    let external_types_data = r#"
    pub struct ExternalC;
    pub struct ExternalD;
    "#;

    let external_types_path = "tests/external_types.rs";
    fs::write(external_types_path, external_types_data)?;

    // build_transitivityを実行（両方のファイルを渡す）
    let result = build_transitivity(&[temp_file_path, external_types_path])?;

    // 生成されたコードを文字列に変換
    let generated_code = result.to_string();

    // InternalA -> ExternalCの変換は生成されるはず（ExternalCは定義された型として認識される）
    assert!(
        generated_code.contains("impl From < InternalA > for ExternalC"),
        "InternalA -> ExternalCの変換が生成されていません"
    );

    // InternalA -> ExternalDの変換も生成されるはず
    assert!(
        generated_code.contains("impl From < InternalA > for ExternalD"),
        "InternalA -> ExternalDの変換が生成されていません"
    );

    // テスト後に一時ファイルを削除
    fs::remove_file(temp_file_path)?;
    fs::remove_file(external_types_path)?;

    Ok(())
}

#[test]
fn test_fully_qualified_from() -> Result<()> {
    // フルパスでFromトレイトを使用するテストデータを作成
    let qualified_test_data = r#"
    pub struct QualifiedA;
    pub struct QualifiedB;
    pub struct QualifiedC;
    
    // 通常のFromトレイト実装
    impl From<QualifiedA> for QualifiedB {
        fn from(_: QualifiedA) -> Self {
            QualifiedB
        }
    }
    
    // フルパスでFromトレイトを指定した実装
    impl ::std::convert::From<QualifiedB> for QualifiedC {
        fn from(_: QualifiedB) -> Self {
            QualifiedC
        }
    }
    "#;

    let temp_file_path = "tests/qualified_test_data.rs";
    fs::write(temp_file_path, qualified_test_data)?;

    // build_transitivityを実行
    let result = build_transitivity(&[temp_file_path])?;

    // 生成されたコードを文字列に変換
    let generated_code = result.to_string();

    // デバッグ用に生成されたコードを出力
    println!("フルパステスト - 生成されたコード:\n{}", generated_code);

    // QualifiedA -> QualifiedCの変換が生成されているか確認
    assert!(
        generated_code.contains("impl From < QualifiedA > for QualifiedC"),
        "QualifiedA -> QualifiedCの変換が生成されていません"
    );

    // テスト後に一時ファイルを削除
    fs::remove_file(temp_file_path)?;

    Ok(())
}

#[test]
fn test_various_from_paths() -> Result<()> {
    // 様々なパスパターンでFromトレイトを使用するテストデータを作成
    let various_paths_data = r#"
    pub struct PathA;
    pub struct PathB;
    pub struct PathC;
    pub struct PathD;
    pub struct PathE;
    
    // 通常のFromトレイト実装
    impl From<PathA> for PathB {
        fn from(_: PathA) -> Self {
            PathB
        }
    }
    
    // std::convert::Fromを使用
    impl std::convert::From<PathB> for PathC {
        fn from(_: PathB) -> Self {
            PathC
        }
    }
    
    // ::std::convert::Fromを使用
    impl ::std::convert::From<PathC> for PathD {
        fn from(_: PathC) -> Self {
            PathD
        }
    }
    
    // 仮想的なモジュールパスを使用
    // 注: 実際にはコンパイルエラーになるが、パーサーレベルではパスとして認識される
    impl some_module::From<PathD> for PathE {
        fn from(_: PathD) -> Self {
            PathE
        }
    }
    "#;

    let temp_file_path = "tests/various_paths_data.rs";
    fs::write(temp_file_path, various_paths_data)?;

    // build_transitivityを実行
    let result = build_transitivity(&[temp_file_path])?;

    // 生成されたコードを文字列に変換
    let generated_code = result.to_string();

    // デバッグ用に生成されたコードを出力
    println!("様々なパスパターン - 生成されたコード:\n{}", generated_code);

    // 各変換が生成されているか確認
    assert!(
        generated_code.contains("impl From < PathA > for PathC"),
        "PathA -> PathCの変換が生成されていません"
    );

    assert!(
        generated_code.contains("impl From < PathA > for PathD"),
        "PathA -> PathDの変換が生成されていません"
    );

    assert!(
        generated_code.contains("impl From < PathA > for PathE"),
        "PathA -> PathEの変換が生成されていません"
    );

    assert!(
        generated_code.contains("impl From < PathB > for PathD"),
        "PathB -> PathDの変換が生成されていません"
    );

    assert!(
        generated_code.contains("impl From < PathB > for PathE"),
        "PathB -> PathEの変換が生成されていません"
    );

    assert!(
        generated_code.contains("impl From < PathC > for PathE"),
        "PathC -> PathEの変換が生成されていません"
    );

    // テスト後に一時ファイルを削除
    fs::remove_file(temp_file_path)?;

    Ok(())
}

#[test]
fn test_generic_types() -> Result<()> {
    // ジェネリック型を使用するテストデータを作成
    let generic_test_data = r#"
    pub struct GenericA<T>(T);
    pub struct GenericB<T>(T);
    pub struct GenericC<T>(T);
    
    // 特定のジェネリックパラメータを持つ型の変換
    impl From<GenericA<String>> for GenericB<String> {
        fn from(value: GenericA<String>) -> Self {
            GenericB(value.0)
        }
    }
    
    impl From<GenericB<String>> for GenericC<String> {
        fn from(value: GenericB<String>) -> Self {
            GenericC(value.0)
        }
    }
    
    // 異なるジェネリックパラメータを持つ型の変換
    impl From<GenericA<i32>> for GenericB<i32> {
        fn from(value: GenericA<i32>) -> Self {
            GenericB(value.0)
        }
    }
    
    // Vec<T>のような標準ライブラリのジェネリック型を使用
    pub struct VecWrapper<T>(Vec<T>);
    pub struct VecProcessor<T>(Vec<T>);
    pub struct VecResult<T>(Vec<T>);
    
    impl From<VecWrapper<String>> for VecProcessor<String> {
        fn from(value: VecWrapper<String>) -> Self {
            VecProcessor(value.0)
        }
    }
    
    impl From<VecProcessor<String>> for VecResult<String> {
        fn from(value: VecProcessor<String>) -> Self {
            VecResult(value.0)
        }
    }
    "#;

    let temp_file_path = "tests/generic_test_data.rs";
    fs::write(temp_file_path, generic_test_data)?;

    // build_transitivityを実行
    let result = build_transitivity(&[temp_file_path])?;

    // 生成されたコードを文字列に変換
    let generated_code = result.to_string();

    // デバッグ用に生成されたコードを出力
    println!(
        "ジェネリック型テスト - 生成されたコード:\n{}",
        generated_code
    );

    // 特定のジェネリックパラメータを持つ型の変換が生成されているか確認
    assert!(
        generated_code.contains("impl From < GenericA < String > > for GenericC < String >"),
        "GenericA<String> -> GenericC<String>の変換が生成されていません"
    );

    // 異なるジェネリックパラメータを持つ型の変換は生成されないことを確認
    // GenericA<i32> -> GenericC<i32>の変換は存在しないはず
    assert!(
        !generated_code.contains("impl From < GenericA < i32 > > for GenericC"),
        "GenericA<i32> -> GenericC<i32>の変換が誤って生成されています"
    );

    // Vec<T>のような標準ライブラリのジェネリック型の変換が生成されているか確認
    assert!(
        generated_code.contains("impl From < VecWrapper < String > > for VecResult < String >"),
        "VecWrapper<String> -> VecResult<String>の変換が生成されていません"
    );

    // テスト後に一時ファイルを削除
    fs::remove_file(temp_file_path)?;

    Ok(())
}
