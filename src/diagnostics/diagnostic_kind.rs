use super::error;

pub enum DiagnosticKind {
    Error(error::Error),
}
