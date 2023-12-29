use std::{result, str::FromStr};

use crate::Token;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed parsing number from: '{1}'")]
    ParseNumber(#[source] <u64 as FromStr>::Err, String),

    #[error("unexpected character: '{0}'")]
    UnexpectedCharacter(char),

    #[error("unexpected EOF")]
    UnexpectedEof,

    #[error("unexpected token: expected a {1}, got {0}")]
    UnexpectedToken(Token, String),
}

pub type Result<T> = result::Result<T, Error>;
