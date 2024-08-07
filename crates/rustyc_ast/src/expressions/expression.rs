use rustyc_span::Span;

use super::ExpressionKind;

#[derive(Clone, Debug)]
pub struct Expression {
    kind: ExpressionKind,
    span: Span,
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn get_kind(&self) -> &ExpressionKind {
        &self.kind
    }

    pub fn get_span(&self) -> &Span {
        &self.span
    }
}
