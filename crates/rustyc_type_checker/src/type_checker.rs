use std::{cell::RefCell, rc::Rc};

use rustyc_hir::Item;
use rustyc_ty::TyContext;

pub struct TypeChecker {
    hir: Rc<Vec<Rc<Item>>>,
    ty_context: Rc<RefCell<TyContext>>,
}

impl TypeChecker {
    pub fn new(hir: Rc<Vec<Rc<Item>>>, ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self { hir, ty_context }
    }

    pub fn check(self) -> rustyc_diagnostics::Result<()> {
        Ok(())
    }
}
