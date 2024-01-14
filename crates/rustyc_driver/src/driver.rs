use rustyc_code_generator::CodeGenerator;
use rustyc_diagnostics::DiagnosticEmitter;
use rustyc_lexer::Lexer;
use rustyc_parser::Parser;

use crate::error;

pub struct Driver {
    source: String,
    diagnostic_emitter: DiagnosticEmitter,
}

impl Driver {
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

    fn run_checked(&mut self) -> rustyc_diagnostics::Result<()> {
        let mut lexer = Lexer::new(&self.source)?;
        let tokens = lexer.lex()?;

        let mut parser = Parser::new(tokens);
        let expression = parser.parse()?;

        let code_generator = CodeGenerator::new(expression);
        code_generator.generate()?;

        Ok(())
    }
}
