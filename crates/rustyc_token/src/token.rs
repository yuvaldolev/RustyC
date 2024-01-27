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

    pub fn glue(&self, joint: &Self) -> Option<Self> {
        let kind = match self.kind {
            TokenKind::Equals => match joint.kind {
                TokenKind::Equals => TokenKind::EqualsEquals,
                _ => return None,
            },
            TokenKind::LessThan => match joint.kind {
                TokenKind::Equals => TokenKind::LessEquals,
                _ => return None,
            },
            TokenKind::GreaterThan => match joint.kind {
                TokenKind::Equals => TokenKind::GreaterEquals,
                _ => return None,
            },
            TokenKind::Not => match joint.kind {
                TokenKind::Equals => TokenKind::NotEquals,
                _ => return None,
            },
            _ => return None,
        };

        Some(Token::new(kind, self.span.to(&joint.span)))
    }
}
