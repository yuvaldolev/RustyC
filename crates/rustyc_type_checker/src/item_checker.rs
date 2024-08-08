use std::{cell::RefCell, rc::Rc};

use rustyc_hir::{FunctionItem, Item, ItemKind};
use rustyc_ty::TyContext;

use crate::function_checker::FunctionChecker;

pub struct ItemChecker {
    item: Rc<Item>,
    ty_context: Rc<RefCell<TyContext>>,
}

impl ItemChecker {
    pub fn new(item: Rc<Item>, ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self { item, ty_context }
    }

    pub fn check(self) -> rustyc_diagnostics::Result<()> {
        match self.item.get_kind() {
            ItemKind::Function(function) => self.check_function(Rc::clone(function)),
        }
    }

    fn check_function(&self, function: Rc<FunctionItem>) -> rustyc_diagnostics::Result<()> {
        let function_checker = FunctionChecker::new(function, Rc::clone(&self.ty_context));
        function_checker.check()
    }
}
