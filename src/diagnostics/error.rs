use std::{result, str::FromStr};

use crate::Token;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed parsing number")]
    ParseNumber(#[source] <u64 as FromStr>::Err),
}

pub type Result<T> = result::Result<T, Error>;
