use std::{cell::RefCell, rc::Rc};

use rustyc_hir::Hir;
use rustyc_hir_walker::HirWalker;
use rustyc_ty::TyContext;

use crate::type_check_visitor::TypeCheckVisitor;

pub struct TypeChecker {
    hir: Rc<Hir>,
    ty_context: Rc<RefCell<TyContext>>,
}

impl TypeChecker {
    pub fn new(hir: Rc<Hir>, ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self { hir, ty_context }
    }

    pub fn check(self) -> rustyc_diagnostics::Result<()> {
        let walker = HirWalker::new();
        let mut type_check_visitor = TypeCheckVisitor::new(Rc::clone(&self.ty_context));
        walker.walk(&self.hir, &mut type_check_visitor)?;

        Ok(())
    }
}
