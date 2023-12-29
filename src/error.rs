use std::{result, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed parsing number from: '{1}'")]
    ParseNumber(#[source] <u64 as FromStr>::Err, String),

    #[error("syntax error: '{0}'")]
    SyntaxError(char),
}

pub type Result<T> = result::Result<T, Error>;
