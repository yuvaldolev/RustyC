use std::fmt;

use crate::{BinaryOperatorToken, DelimiterToken};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TokenKind {
    Equal,
    EqualEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    Not,
    NotEqual,
    Semicolon,
    Number(u64),
    Identifier(String),
    BinaryOperator(BinaryOperatorToken),
    OpenDelimiter(DelimiterToken),
    CloseDelimiter(DelimiterToken),
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equal => write!(f, "="),
            Self::EqualEqual => write!(f, "=="),
            Self::LessThan => write!(f, "<"),
            Self::LessEqual => write!(f, "<="),
            Self::GreaterThan => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
            Self::Not => write!(f, "!"),
            Self::NotEqual => write!(f, "!="),
            Self::Semicolon => write!(f, ";"),
            Self::Number(value) => write!(f, "{}", value),
            Self::Identifier(name) => write!(f, "{}", name),
            Self::BinaryOperator(BinaryOperatorToken::Plus) => write!(f, "+"),
            Self::BinaryOperator(BinaryOperatorToken::Minus) => write!(f, "-"),
            Self::BinaryOperator(BinaryOperatorToken::Star) => write!(f, "*"),
            Self::BinaryOperator(BinaryOperatorToken::Slash) => write!(f, "/"),
            Self::OpenDelimiter(DelimiterToken::Parenthesis) => write!(f, "("),
            Self::CloseDelimiter(DelimiterToken::Parenthesis) => write!(f, ")"),
            Self::OpenDelimiter(DelimiterToken::Brace) => write!(f, "{{"),
            Self::CloseDelimiter(DelimiterToken::Brace) => write!(f, "}}"),
            Self::Eof => write!(f, "<eof>"),
        }
    }
}
