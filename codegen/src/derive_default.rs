use syn::{Ident, Result, Type};

#[cfg(test)]
mod tests;

static DEFAULT_TYPE_NAMES: &[&str] = &[
    "String", "Vec", "Option", "bool", "i32", "u32", "u64", "f64", "HashMap", "BTreeMap",
];

/// 外部の型について `Default` が実装されているかどうかを判定する
/// derive_default_typesに入力するファイルで定義された型についてはfalseを返す
fn is_default_types(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            let last_segment = type_path.path.segments.last().unwrap();
            let type_name = last_segment.ident.to_string();
            DEFAULT_TYPE_NAMES.contains(&type_name.as_str())
        }
        _ => false,
    }
}

/// file定義された型のうち、`#[derive(Default)]` を指定可能な型を返す
///
/// それは全てのフィールドが次のいずれかの条件を満たす構造体である
///
/// - is_default_typesがtrueを返す型
/// - derive_default_typesの結果に含まれる型
pub fn derive_default_types(file: &syn::File) -> Result<Vec<Ident>> {
    let mut default_types = Vec::new();

    for item in &file.items {
        if let syn::Item::Struct(item_struct) = item {
            let mut can_derive_default = true;

            // 構造体のフィールドをチェック
            match &item_struct.fields {
                syn::Fields::Named(fields_named) => {
                    for field in &fields_named.named {
                        if !is_default_types(&field.ty) {
                            can_derive_default = false;
                            break;
                        }
                    }
                }
                syn::Fields::Unnamed(fields_unnamed) => {
                    for field in &fields_unnamed.unnamed {
                        if !is_default_types(&field.ty) {
                            can_derive_default = false;
                            break;
                        }
                    }
                }
                syn::Fields::Unit => {}
            }

            if can_derive_default {
                default_types.push(item_struct.ident.clone());
            }
        }
    }

    Ok(default_types)
}
