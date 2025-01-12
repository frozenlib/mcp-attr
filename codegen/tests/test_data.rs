// テスト用の型定義
pub struct TypeA;
pub struct TypeB;
pub struct TypeC;
pub struct TypeD;

// 基本的な変換の実装
impl From<TypeA> for TypeB {
    fn from(_: TypeA) -> Self {
        TypeB
    }
}

impl From<TypeB> for TypeC {
    fn from(_: TypeB) -> Self {
        TypeC
    }
}

impl From<TypeC> for TypeD {
    fn from(_: TypeC) -> Self {
        TypeD
    }
}
