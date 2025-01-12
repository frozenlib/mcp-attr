use quote::ToTokens;
use syn::{parse_quote, Ident};

#[test]
fn test_add_derive_to_struct() -> syn::Result<()> {
    // 入力となるコードを準備
    let mut file: syn::File = parse_quote! {
        struct MyStruct {
            field: i32
        }

        struct OtherStruct {
            field: String
        }
    };

    // MyStructにのみderiveを追加
    let types = vec![Ident::new("MyStruct", proc_macro2::Span::call_site())];
    let traits = vec!["Debug", "Clone"];

    super::add_derive(&mut file, &types, &traits)?;

    // 期待される出力を準備
    let expected: syn::File = parse_quote! {
        #[derive(Debug, Clone)]
        struct MyStruct {
            field: i32
        }

        struct OtherStruct {
            field: String
        }
    };

    assert_eq!(
        file.to_token_stream().to_string(),
        expected.to_token_stream().to_string()
    );
    Ok(())
}

#[test]
fn test_add_derive_to_multiple_structs() -> syn::Result<()> {
    let mut file: syn::File = parse_quote! {
        struct First {
            x: i32
        }

        struct Second {
            y: String
        }

        struct Third {
            z: bool
        }
    };

    let types = vec![
        Ident::new("First", proc_macro2::Span::call_site()),
        Ident::new("Second", proc_macro2::Span::call_site()),
    ];
    let traits = vec!["Debug", "Clone", "PartialEq"];

    super::add_derive(&mut file, &types, &traits)?;

    let expected: syn::File = parse_quote! {
        #[derive(Debug, Clone, PartialEq)]
        struct First {
            x: i32
        }

        #[derive(Debug, Clone, PartialEq)]
        struct Second {
            y: String
        }

        struct Third {
            z: bool
        }
    };

    assert_eq!(
        file.to_token_stream().to_string(),
        expected.to_token_stream().to_string()
    );
    Ok(())
}

#[test]
fn test_add_derive_with_existing_attributes() -> syn::Result<()> {
    let mut file: syn::File = parse_quote! {
        #[derive(Default)]
        struct Target {
            field: i32
        }
    };

    let types = vec![Ident::new("Target", proc_macro2::Span::call_site())];
    let traits = vec!["Debug", "Clone"];

    super::add_derive(&mut file, &types, &traits)?;

    let expected: syn::File = parse_quote! {
        #[derive(Debug, Clone)]
        #[derive(Default)]
        struct Target {
            field: i32
        }
    };

    assert_eq!(
        file.to_token_stream().to_string(),
        expected.to_token_stream().to_string()
    );
    Ok(())
}
