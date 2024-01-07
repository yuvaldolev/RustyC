use std::{result, str::FromStr};

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("failed parsing number")]
    ParseNumber(#[source] <u64 as FromStr>::Err),

    #[error("unknown token start")]
    UnknownTokenStart,

    #[error("aborting due to {0} pervious errors")]
    Aborted(u32),
}

pub type Result<T> = result::Result<T, Error>;
