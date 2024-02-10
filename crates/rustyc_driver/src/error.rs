use std::result;

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("aborting due to {0} previous errors")]
    Aborted(u32),
}

pub type Result<T> = result::Result<T, Error>;
