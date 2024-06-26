use rustyc_span::Span;

use crate::{diagnostic_kind::DiagnosticKind, error};

#[derive(Debug)]
pub struct Diagnostic {
    kind: DiagnosticKind,
    span: Span,
}

impl Diagnostic {
    pub fn new_error(e: error::Error, span: Span) -> Self {
        Self {
            kind: DiagnosticKind::Error(e),
            span,
        }
    }

    pub fn get_kind(&self) -> &DiagnosticKind {
        &self.kind
    }

    pub fn get_span(&self) -> &Span {
        &self.span
    }
}
