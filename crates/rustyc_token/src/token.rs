use rustyc_span::Span;

use crate::TokenKind;

#[derive(Clone, Debug)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn new_eof() -> Self {
        Self::new(TokenKind::Eof, Span::new_dummy())
    }

    pub fn get_kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn get_span(&self) -> &Span {
        &self.span
    }
}
