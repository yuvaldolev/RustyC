use crate::{
    diagnostics::{self, DiagnosticEmitter},
    lexer::Lexer,
    parser::Parser,
};

use super::error;

pub struct Compiler {
    source: String,
    diagnostic_emitter: DiagnosticEmitter,
}

impl Compiler {
    pub fn new(source: String) -> Self {
        let diagnostic_emitter = DiagnosticEmitter::new(source.clone());

        Self {
            source,
            diagnostic_emitter,
        }
    }

    pub fn run(&mut self) -> error::Result<()> {
        self.run_checked().map_err(|diagnostic| {
            self.diagnostic_emitter.emit(diagnostic);
            error::Error::Aborted(self.diagnostic_emitter.get_error_count())
        })
    }

    fn run_checked(&mut self) -> diagnostics::Result<()> {
        let mut lexer = Lexer::new(&self.source)?;
        let tokens = lexer.lex()?;

        let mut parser = Parser::new(tokens);
        let expression = parser.parse()?;

        Ok(())
    }
}
