use crate::TyId;

#[derive(Eq, Hash, PartialEq)]
pub enum Ty {
    Int,
    Pointer(TyId),
}
