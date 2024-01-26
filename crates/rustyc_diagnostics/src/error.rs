use std::str::FromStr;

use rustyc_token::{TokenKind, TokenKindSet};

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("failed parsing number")]
    ParseNumber(#[source] <u64 as FromStr>::Err),

    #[error("unknown token start")]
    UnknownTokenStart,

    #[error("unexpected token '{0}', expected one of: {1}")]
    UnexpectedToken(TokenKind, TokenKindSet),

    #[error("expected an expression")]
    ExpressionExpected,

    #[error("parsing completed without reaching EOF")]
    EofExpected,

    #[error("invalid expression")]
    InvalidExpression,
}
