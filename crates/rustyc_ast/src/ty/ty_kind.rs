use super::Ty;

#[derive(Clone)]
pub enum TyKind {
    Identifier(String),
    Pointer(Box<Ty>),
}
