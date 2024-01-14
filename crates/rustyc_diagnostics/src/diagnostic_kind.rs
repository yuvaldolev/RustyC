use crate::error;

pub enum DiagnosticKind {
    Error(error::Error),
}
