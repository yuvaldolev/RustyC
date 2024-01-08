use std::result;

use super::Diagnostic;

pub type Result<T> = result::Result<T, Diagnostic>;
