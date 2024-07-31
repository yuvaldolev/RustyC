use std::rc::Rc;

use crate::statement_lowerer::StatementLowerer;

pub struct BlockLowerer {
    block: Rc<rustyc_ast::Block>,
}

impl BlockLowerer {
    pub fn new(block: Rc<rustyc_ast::Block>) -> Self {
        Self { block }
    }

    pub fn lower(self) -> Rc<rustyc_hir::Block> {
        let statements: Vec<Rc<rustyc_hir::Statement>> = self
            .block
            .get_statements()
            .iter()
            .map(|statement| Self::lower_statement(Rc::clone(statement)))
            .collect();

        Rc::new(rustyc_hir::Block::new(
            statements,
            self.block.get_span().clone(),
        ))
    }

    pub fn lower_statement(statement: Rc<rustyc_ast::Statement>) -> Rc<rustyc_hir::Statement> {
        let statement_lowerer = StatementLowerer::new(statement);
        statement_lowerer.lower()
    }
}
