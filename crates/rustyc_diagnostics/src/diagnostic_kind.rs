use crate::error;

#[derive(Debug)]
pub enum DiagnosticKind {
    Error(error::Error),
}
