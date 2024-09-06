use std::{cell::RefCell, rc::Rc};

use rustyc_ty::TyContext;

use crate::{
    function_lowering_context::FunctionLoweringContext, statement_lowerer::StatementLowerer,
};

pub struct BlockLowerer {
    context: Rc<FunctionLoweringContext>,
    block: Rc<rustyc_ast::Block>,
    ty_context: Rc<RefCell<TyContext>>,
}

impl BlockLowerer {
    pub fn new(
        context: Rc<FunctionLoweringContext>,
        block: Rc<rustyc_ast::Block>,
        ty_context: Rc<RefCell<TyContext>>,
    ) -> Self {
        Self {
            context,
            block,
            ty_context,
        }
    }

    pub fn lower(self) -> rustyc_diagnostics::Result<Rc<rustyc_hir::Block>> {
        let statements: Vec<Rc<rustyc_hir::statements::Statement>> = self
            .block
            .get_statements()
            .iter()
            .map(|statement| self.lower_statement(Rc::clone(statement)))
            .collect::<rustyc_diagnostics::Result<Vec<_>>>()?;

        Ok(Rc::new(rustyc_hir::Block::new(
            statements,
            self.block.get_span().clone(),
        )))
    }

    pub fn lower_statement(
        &self,
        statement: Rc<rustyc_ast::statements::Statement>,
    ) -> rustyc_diagnostics::Result<Rc<rustyc_hir::statements::Statement>> {
        let statement_lowerer = StatementLowerer::new(
            Rc::clone(&self.context),
            statement,
            Rc::clone(&self.ty_context),
        );
        statement_lowerer.lower()
    }
}
