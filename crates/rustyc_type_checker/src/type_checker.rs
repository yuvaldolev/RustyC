use std::{cell::RefCell, rc::Rc};

use rustyc_hir::Hir;
use rustyc_ty::TyContext;

use crate::item_checker::ItemChecker;

pub struct TypeChecker {
    hir: Rc<Hir>,
    ty_context: Rc<RefCell<TyContext>>,
}

impl TypeChecker {
    pub fn new(hir: Rc<Hir>, ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self { hir, ty_context }
    }

    pub fn check(self) -> rustyc_diagnostics::Result<()> {
        for item in self.hir.get_items().iter() {
            let item_checker = ItemChecker::new(Rc::clone(item), Rc::clone(&self.ty_context));
            item_checker.check()?;
        }

        Ok(())
    }
}
