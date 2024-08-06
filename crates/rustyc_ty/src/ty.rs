use crate::TyId;

#[derive(Eq, Hash, PartialEq)]
pub enum Ty {
    Int,
    // TODO: Should rc::Weak be used here?
    Pointer(TyId),
}
