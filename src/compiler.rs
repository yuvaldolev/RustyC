use crate::{
    diagnostics::{self, DiagnosticEmitter},
    lexer::Lexer,
};

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

    pub fn run(&mut self) -> diagnostics::Result<()> {
        self.run_checked()
            .map_err(|_| diagnostics::Error::Aborted(self.diagnostic_emitter.get_error_count()))
    }

    fn run_checked(&mut self) -> diagnostics::Result<()> {
        let mut lexer = Lexer::new(&self.source, &mut self.diagnostic_emitter);
        let tokens = lexer.lex()?;
        println!("Tokens: {}", tokens.len());

        Ok(())
    }
}
