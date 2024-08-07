use std::{cell::RefCell, rc::Rc};

use rustyc_ast_lowerer::AstLowerer;
use rustyc_code_generator::CodeGenerator;
use rustyc_diagnostics::DiagnosticEmitter;
use rustyc_lexer::Lexer;
use rustyc_parser::Parser;
use rustyc_ty::TyContext;
use rustyc_type_checker::TypeChecker;

use crate::error;

// TODO: Compiler panics when runs follows: `cargo run -- "{ i = 55; }"`
// TODO: Compiler panics when runs follows: `cargo run -- "ain() { x=3; return *&x; } => 3"`

pub struct Driver {
    source: String,
    diagnostic_emitter: DiagnosticEmitter,
    ty_context: Rc<RefCell<TyContext>>,
}

impl Driver {
    pub fn new(source: String) -> Self {
        let diagnostic_emitter = DiagnosticEmitter::new(source.clone());

        Self {
            source,
            diagnostic_emitter,
            ty_context: Rc::new(RefCell::new(TyContext::new())),
        }
    }

    pub fn run(&mut self) -> error::Result<()> {
        self.run_passes().map_err(|diagnostic| {
            self.diagnostic_emitter.emit(diagnostic);
            error::Error::Aborted(self.diagnostic_emitter.get_error_count())
        })
    }

    fn run_passes(&mut self) -> rustyc_diagnostics::Result<()> {
        let lexer = Lexer::new(&self.source)?;
        let tokens = lexer.lex()?;

        let parser = Parser::new(tokens);
        let ast = parser.parse()?;

        let ast_lowerer = AstLowerer::new(ast, Rc::clone(&self.ty_context));
        let hir = ast_lowerer.lower();

        let type_checker = TypeChecker::new(Rc::clone(&hir), Rc::clone(&self.ty_context));
        type_checker.check()?;

        let code_generator = CodeGenerator::new(hir);
        code_generator.generate()?;

        Ok(())
    }
}
