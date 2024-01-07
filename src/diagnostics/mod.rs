mod diagnostic;
mod diagnostic_emitter;
mod diagnostic_kind;
mod error;

pub use diagnostic::Diagnostic;
pub use diagnostic_emitter::DiagnosticEmitter;
pub use error::{Error, Result};
