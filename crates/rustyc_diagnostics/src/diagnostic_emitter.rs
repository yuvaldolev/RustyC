use std::error::Error;

use crate::{diagnostic_kind::DiagnosticKind, error, Diagnostic};

pub struct DiagnosticEmitter {
    source: String,
    error_count: u32,
}

impl DiagnosticEmitter {
    pub fn new(source: String) -> Self {
        Self {
            source,
            error_count: 0,
        }
    }

    pub fn get_error_count(&self) -> u32 {
        self.error_count
    }

    pub fn emit(&mut self, diagnostic: Diagnostic) {
        match diagnostic.get_kind() {
            DiagnosticKind::Error(e) => self.emit_error_diagnostic(&diagnostic, e),
        }
    }

    fn emit_error_diagnostic(&mut self, diagnostic: &Diagnostic, e: &error::Error) {
        let mut error_message = format!("error: {}", e);
        if let Some(source) = e.source() {
            error_message = format!("{error_message}: {source}")
        }
        eprintln!("{}", error_message);

        eprintln!("{}", self.source);

        let span = diagnostic.get_span();
        eprintln!(
            "{}{}",
            " ".repeat(span.get_low()),
            "^".repeat(span.get_high() - span.get_low())
        );

        self.error_count += 1;
    }
}
