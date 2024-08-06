use std::rc::Rc;

use rustyc_hir::Item;

pub struct TypeChecker {
    hir: Vec<Rc<Item>>,
}

impl TypeChecker {
    pub fn new(hir: Vec<Rc<Item>>) -> Self {
        Self { hir }
    }

    pub fn check(self) -> rustyc_diagnostics::Result<()> {
        Ok(())
    }
}
