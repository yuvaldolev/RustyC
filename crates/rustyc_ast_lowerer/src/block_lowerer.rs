use std::{cell::RefCell, rc::Rc};

use rustyc_ty::TyContext;

use crate::statement_lowerer::StatementLowerer;

pub struct BlockLowerer {
    block: Rc<rustyc_ast::Block>,
    ty_context: Rc<RefCell<TyContext>>,
}

impl BlockLowerer {
    pub fn new(block: Rc<rustyc_ast::Block>, ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self { block, ty_context }
    }

    pub fn lower(self) -> Rc<rustyc_hir::Block> {
        let statements: Vec<Rc<rustyc_hir::statements::Statement>> = self
            .block
            .get_statements()
            .iter()
            .map(|statement| self.lower_statement(Rc::clone(statement)))
            .collect();

        Rc::new(rustyc_hir::Block::new(
            statements,
            self.block.get_span().clone(),
        ))
    }

    pub fn lower_statement(
        &self,
        statement: Rc<rustyc_ast::statements::Statement>,
    ) -> Rc<rustyc_hir::statements::Statement> {
        let statement_lowerer = StatementLowerer::new(statement, Rc::clone(&self.ty_context));
        statement_lowerer.lower()
    }
}
