use super::*;

#[test]
fn test_derive_default_types_empty_struct() {
    let code = r#"
        struct Empty {}
    "#;
    let file = syn::parse_str(code).unwrap();
    let result = derive_default_types(&file).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].to_string(), "Empty");
}

#[test]
fn test_derive_default_types_with_default_fields() {
    let code = r#"
        struct User {
            name: String,
            emails: Vec<String>,
            age: Option<String>,
        }
    "#;
    let file = syn::parse_str(code).unwrap();
    let result = derive_default_types(&file).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].to_string(), "User");
}

#[test]
fn test_derive_default_types_with_non_default_fields() {
    let code = r#"
        struct NonDefault {
            custom: CustomType,
        }
    "#;
    let file = syn::parse_str(code).unwrap();
    let result = derive_default_types(&file).unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_derive_default_types_tuple_struct() {
    let code = r#"
        struct TupleStruct(String, Vec<String>);
    "#;
    let file = syn::parse_str(code).unwrap();
    let result = derive_default_types(&file).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].to_string(), "TupleStruct");
}

#[test]
fn test_derive_default_types_unit_struct() {
    let code = r#"
        struct UnitStruct;
    "#;
    let file = syn::parse_str(code).unwrap();
    let result = derive_default_types(&file).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].to_string(), "UnitStruct");
}
