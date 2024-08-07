use std::fmt;

use crate::{Keyword, TokenKind};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TokenCategory {
    Token(TokenKind),
    Keyword(Keyword),
    Identifier,
}

impl fmt::Display for TokenCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenCategory::Token(kind) => write!(f, "`{kind}`"),
            TokenCategory::Keyword(keyword) => {
                write!(f, "`{}`", keyword.to_string().to_lowercase())
            }
            TokenCategory::Identifier => write!(f, "identifier"),
        }
    }
}
