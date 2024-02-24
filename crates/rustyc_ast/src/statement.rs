use rustyc_span::Span;

use crate::statement_kind::StatementKind;

pub struct Statement {
    kind: StatementKind,
    span: Span,
}

impl Statement {
    pub fn new(kind: StatementKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn get_kind(&self) -> &StatementKind {
        &self.kind
    }

    pub fn get_span(&self) -> &Span {
        &self.span
    }
}
