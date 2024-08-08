use std::{cell::RefCell, rc::Rc};

use rustyc_hir::Block;
use rustyc_ty::TyContext;

use crate::statement_checker::StatementChecker;

pub struct BlockChecker {
    block: Rc<Block>,
    ty_context: Rc<RefCell<TyContext>>,
}

impl BlockChecker {
    pub fn new(block: Rc<Block>, ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self { block, ty_context }
    }

    pub fn check(self) -> rustyc_diagnostics::Result<()> {
        for statement in self.block.get_statements().iter() {
            let statement_checker =
                StatementChecker::new(Rc::clone(statement), Rc::clone(&self.ty_context));
            statement_checker.check()?;
        }

        Ok(())
    }
}
