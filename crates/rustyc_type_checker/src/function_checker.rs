use std::{cell::RefCell, rc::Rc};

use rustyc_hir::FunctionItem;
use rustyc_ty::TyContext;

use crate::block_checker::BlockChecker;

pub struct FunctionChecker {
    function: Rc<FunctionItem>,
    ty_context: Rc<RefCell<TyContext>>,
}

impl FunctionChecker {
    pub fn new(function: Rc<FunctionItem>, ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self {
            function,
            ty_context,
        }
    }

    pub fn check(self) -> rustyc_diagnostics::Result<()> {
        let block_checker =
            BlockChecker::new(self.function.get_body(), Rc::clone(&self.ty_context));
        block_checker.check()
    }
}
