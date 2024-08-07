use std::fmt;

use crate::TokenKind;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TokenCategory {
    Token(TokenKind),
    Identifier,
}

impl fmt::Display for TokenCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenCategory::Token(kind) => write!(f, "`{kind}`"),
            TokenCategory::Identifier => write!(f, "identifier"),
        }
    }
}
