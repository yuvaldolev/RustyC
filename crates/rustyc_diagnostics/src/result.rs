use std::result;

use crate::Diagnostic;

pub type Result<T> = result::Result<T, Diagnostic>;
