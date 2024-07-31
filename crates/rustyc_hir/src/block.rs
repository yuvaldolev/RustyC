use std::rc::Rc;

use rustyc_span::Span;

use crate::Statement;

pub struct Block {
    statements: Vec<Rc<Statement>>,
    span: Span,
}

impl Block {
    pub fn new(statements: Vec<Rc<Statement>>, span: Span) -> Self {
        Self { statements, span }
    }
}
