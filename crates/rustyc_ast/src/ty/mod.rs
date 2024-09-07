mod ty_kind;

pub use ty_kind::TyKind;

use rustyc_span::Span;

#[derive(Clone)]
pub struct Ty {
    kind: TyKind,
    span: Span,
}

impl Ty {
    pub fn new(kind: TyKind, span: Span) -> Self {
        Self { kind, span }
    }
}
