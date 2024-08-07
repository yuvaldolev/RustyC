use std::str::FromStr;

use rustyc_token::{TokenCategory, TokenCategorySet, TokenKind};

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("failed parsing number")]
    ParseNumber(#[source] <u64 as FromStr>::Err),

    #[error("unknown token start")]
    UnknownTokenStart,

    #[error("unexpected token `{0}`, expected: {1}")]
    UnexpectedTokenSingle(TokenKind, TokenCategory),

    #[error("unexpected token `{0}`, expected one of: {1}")]
    UnexpectedTokenMultiple(TokenKind, TokenCategorySet),

    #[error("expected an expression")]
    ExpressionExpected,

    #[error("parsing completed without reaching EOF")]
    EofExpected,

    #[error("invalid statement")]
    InvalidStatement,

    #[error("invalid expression")]
    InvalidExpression,

    #[error("invalid expression statement")]
    InvalidExpressionStatement,

    #[error("invalid negation expression")]
    InvalidNegationExpression,

    #[error("invalid assignment expression")]
    InvalidAssignmentExpression,

    #[error("not an lvalue")]
    NotAnLvalue,
}
