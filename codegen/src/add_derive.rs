use syn::{parse_quote, Attribute, Ident, Item};

#[cfg(test)]
mod tests;

/// fileで定義された型のうち、typesで指定されたものについて `#[derive(Trait0,Trait1)]` のようなderiveを追加します。
/// deriveの中身は引数traitsで指定します。
pub fn add_derive(file: &mut syn::File, types: &[Ident], traits: &[&str]) -> syn::Result<()> {
    // 各アイテムに対して処理を行う
    for item in &mut file.items {
        if let Item::Struct(item_struct) = item {
            // 型名が指定されたtypesに含まれているかチェック
            if types.iter().any(|t| t == &item_struct.ident) {
                // トレイト名をIdentに変換
                let trait_idents: Vec<Ident> = traits
                    .iter()
                    .map(|t| Ident::new(t, proc_macro2::Span::call_site()))
                    .collect();

                // derive属性を作成
                let derive_attr: Attribute = parse_quote! {
                    #[derive(#(#trait_idents),*)]
                };

                // 既存の属性リストの先頭にderive属性を追加
                item_struct.attrs.insert(0, derive_attr);
            }
        }
    }
    Ok(())
}
