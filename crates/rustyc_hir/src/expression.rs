use std::rc::Rc;

use rustyc_span::Span;
use rustyc_ty::Ty;

use crate::ExpressionKind;

pub struct Expression {
    kind: ExpressionKind,
    ty: Rc<Ty>,
    span: Span,
}

impl Expression {
    pub fn new(kind: ExpressionKind, ty: Rc<Ty>, span: Span) -> Self {
        Self { kind, ty, span }
    }

    pub fn get_kind(&self) -> &ExpressionKind {
        &self.kind
    }

    pub fn get_ty(&self) -> Rc<Ty> {
        Rc::clone(&self.ty)
    }

    pub fn get_span(&self) -> &Span {
        &self.span
    }
}
