use super::*;
use syn::parse_quote;

#[test]
fn test_basic_transitivity() -> Result<()> {
    let input = parse_quote! {
        struct A;
        struct B;
        struct C;

        impl From<A> for B {
            fn from(a: A) -> Self {
                B
            }
        }

        impl From<B> for C {
            fn from(b: B) -> Self {
                C
            }
        }
    };

    let output = build_transitivity_raw(&[input])?;
    let output_str = output.to_string();

    println!("Generated code:\n{}", output_str);
    // 既存の変換は生成しない
    assert!(output_str.contains("impl :: std :: convert :: From < A > for C"));
    Ok(())
}

#[test]
fn test_self_keyword() -> Result<()> {
    let input = parse_quote! {
        struct A;
        struct B;
        struct C;

        impl From<A> for B {
            fn from(a: A) -> Self {
                B
            }
        }

        impl From<&Self> for B {
            fn from(_: &Self) -> Self {
                B
            }
        }

        impl From<B> for C {
            fn from(_: B) -> Self {
                C
            }
        }
    };

    let output = build_transitivity_raw(&[input])?;
    let output_str = output.to_string();

    println!("Generated code:\n{}", output_str);
    // 既存の変換は生成しない
    // A -> C と &B -> C の推移的な変換のみが生成される
    assert!(output_str.contains("impl :: std :: convert :: From < A > for C"));
    assert!(output_str.contains("impl :: std :: convert :: From < & B > for C"));
    Ok(())
}

#[test]
fn test_skip_generic_types() -> Result<()> {
    let input = parse_quote! {
        struct A;
        struct B;
        struct C<T>;

        impl From<A> for B {
            fn from(a: A) -> Self {
                B
            }
        }

        impl<T> From<B> for C<T> {
            fn from(_: B) -> Self {
                unimplemented!()
            }
        }
    };

    let output = build_transitivity_raw(&[input])?;
    let output_str = output.to_string();

    println!("Generated code:\n{}", output_str);
    // ジェネリック型への変換はスキップされるため、A -> C<T> の変換は生成されない
    assert!(!output_str.contains("impl :: std :: convert :: From < A > for C"));
    Ok(())
}

#[test]
fn test_multiple_files() -> Result<()> {
    let input1 = parse_quote! {
        struct A;
        struct B;

        impl From<A> for B {
            fn from(a: A) -> Self {
                B
            }
        }
    };

    let input2 = parse_quote! {
        struct C;

        impl From<B> for C {
            fn from(b: B) -> Self {
                C
            }
        }
    };

    let output = build_transitivity_raw(&[input1, input2])?;
    let output_str = output.to_string();

    println!("Generated code:\n{}", output_str);
    // 既存の変換は生成しない
    assert!(output_str.contains("impl :: std :: convert :: From < A > for C"));
    Ok(())
}

#[test]
fn test_ref_self_transitivity() -> Result<()> {
    let input = parse_quote! {
        struct B;
        struct C;

        impl From<&Self> for B {
            fn from(_: &Self) -> Self {
                B
            }
        }

        impl From<B> for C {
            fn from(_: B) -> Self {
                C
            }
        }
    };

    let output = build_transitivity_raw(&[input])?;
    let output_str = output.to_string();

    println!("Generated code:\n{}", output_str);
    // 既存の変換は生成しない
    assert!(output_str.contains("impl :: std :: convert :: From < & B > for C"));
    Ok(())
}

#[test]
fn test_no_extra_conversions() -> Result<()> {
    let input = parse_quote! {
        struct B;
        struct C;

        impl From<&Self> for B {
            fn from(_: &Self) -> Self {
                B
            }
        }

        impl From<B> for C {
            fn from(_: B) -> Self {
                C
            }
        }
    };

    let output = build_transitivity_raw(&[input])?;
    let output_str = output.to_string();

    println!("Generated code:\n{}", output_str);

    // 期待される変換のみが生成されていることを確認
    let expected_conversions = [
        "impl :: std :: convert :: From < & B > for C { fn from (value : & B) -> Self { < C as :: std :: convert :: From < B >> :: from (< B as :: std :: convert :: From < & B >> :: from (value)) } }",
    ];

    // 生成されたコードから空白を除去
    let normalized_output = output_str.replace(" ", "");
    let expected_normalized = expected_conversions.join("").replace(" ", "");

    // 生成されたコードと期待されるコードが完全に一致することを確認
    assert_eq!(
        normalized_output, expected_normalized,
        "余計な変換が生成されています"
    );

    Ok(())
}

#[test]
fn test_multiple_paths() -> Result<()> {
    let input = parse_quote! {
        struct A;
        struct B;
        struct C;
        struct D;

        // A -> B -> D のパス
        impl From<A> for B {
            fn from(_: A) -> Self {
                B
            }
        }
        impl From<B> for D {
            fn from(_: B) -> Self {
                D
            }
        }

        // A -> C -> D の別のパス
        impl From<A> for C {
            fn from(_: A) -> Self {
                C
            }
        }
        impl From<C> for D {
            fn from(_: C) -> Self {
                D
            }
        }
    };

    let output = build_transitivity_raw(&[input])?;
    let output_str = output.to_string();

    println!("Generated code:\n{}", output_str);
    // A -> D への変換は複数のパスがあるため生成されない
    assert!(!output_str.contains("impl :: std :: convert :: From < A > for D"));
    Ok(())
}
