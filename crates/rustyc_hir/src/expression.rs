use rustyc_span::Span;
use rustyc_ty::TyId;

use crate::ExpressionKind;

pub struct Expression {
    kind: ExpressionKind,
    ty: TyId,
    span: Span,
}

impl Expression {
    pub fn new(kind: ExpressionKind, ty: TyId, span: Span) -> Self {
        Self { kind, ty, span }
    }

    pub fn get_kind(&self) -> &ExpressionKind {
        &self.kind
    }

    pub fn get_ty(&self) -> TyId {
        self.ty
    }

    pub fn get_span(&self) -> &Span {
        &self.span
    }
}
