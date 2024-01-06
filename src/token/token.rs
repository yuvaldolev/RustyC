use crate::{span::Span, token_kind::TokenKind};

pub struct Token {
    kind: TokenKind,
    span: Span,
}
