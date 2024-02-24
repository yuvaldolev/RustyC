use rustyc_span::Span;

use crate::statement::Statement;

pub struct Block {
    statements: Vec<Statement>,
    span: Span,
}

impl Block {
    pub fn new(statements: Vec<Statement>, span: Span) -> Self {
        Self { statements, span }
    }

    pub fn get_statements(&self) -> &[Statement] {
        &self.statements
    }
}
