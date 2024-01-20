use std::str::FromStr;

use rustyc_token::{TokenCategory, TokenKind, TokenKindSet};

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("failed parsing number")]
    ParseNumber(#[source] <u64 as FromStr>::Err),

    #[error("unknown token start")]
    UnknownTokenStart,

    #[error("unexpected token '{0}', expected one of: {1}")]
    UnexpectedToken(TokenKind, TokenKindSet),

    #[error("unexpcted token '{0}', expected a {1} token")]
    TokenNotOfCategory(TokenKind, TokenCategory),

    #[error("invalid expression")]
    InvalidExpression,

    #[error("parsing completed without reaching EOF")]
    EofExpected,
}
