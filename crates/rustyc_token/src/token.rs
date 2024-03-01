use rustyc_span::Span;

use crate::{Keyword, TokenKind};

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
            TokenKind::Equal => match joint.kind {
                TokenKind::Equal => TokenKind::EqualEqual,
                _ => return None,
            },
            TokenKind::LessThan => match joint.kind {
                TokenKind::Equal => TokenKind::LessEqual,
                _ => return None,
            },
            TokenKind::GreaterThan => match joint.kind {
                TokenKind::Equal => TokenKind::GreaterEqual,
                _ => return None,
            },
            TokenKind::Not => match joint.kind {
                TokenKind::Equal => TokenKind::NotEqual,
                _ => return None,
            },
            _ => return None,
        };

        Some(Token::new(kind, self.span.to(&joint.span)))
    }

    pub fn is_keyword(&self, keyword: &Keyword) -> bool {
        self.get_identifier()
            .is_some_and(|identifier| keyword.to_string().to_lowercase() == identifier)
    }

    pub fn get_identifier(&self) -> Option<&str> {
        match &self.kind {
            TokenKind::Identifier(name) => Some(name),
            _ => None,
        }
    }
}
