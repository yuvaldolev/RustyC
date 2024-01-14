use std::str::FromStr;

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("failed parsing number")]
    ParseNumber(#[source] <u64 as FromStr>::Err),

    #[error("unknown token start")]
    UnknownTokenStart,

    #[error("unexpected EOF")]
    UnexpectedEof,

    // TODO: Add expected token kind to here once token formatting is implemented.
    #[error("unexpected token")]
    UnexpectedToken,

    #[error("invalid expression")]
    InvalidExpression,
}
