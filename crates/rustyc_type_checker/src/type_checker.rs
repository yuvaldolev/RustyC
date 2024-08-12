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
        let visitor = HirVisitor::default();
        visitor.set_item_visitor(Box::new(ItemAnalyzer::new()));
        visitor.visit(self.hir)?;

        let hir_traverser = HirTraverser::new(Rc::clone(&self.hir));
        hir_traverser.register_item_handler(ItemKind::Function, |item, function| Ok(()))?;
        for item in self.hir.get_items().iter() {
            let item_checker = ItemChecker::new(Rc::clone(item), Rc::clone(&self.ty_context));
            item_checker.check()?;
        }

        Ok(())
    }
}
