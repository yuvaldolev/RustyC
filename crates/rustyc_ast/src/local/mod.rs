mod local_kind;

pub use local_kind::LocalKind;

use crate::ty::Ty;

#[derive(Clone)]
pub struct Local {
    kind: LocalKind,
    identifier: String,
    ty: Ty,
}

impl Local {
    pub fn new(kind: LocalKind, identifier: String, ty: Ty) -> Self {
        Self {
            kind,
            identifier,
            ty,
        }
    }
}
