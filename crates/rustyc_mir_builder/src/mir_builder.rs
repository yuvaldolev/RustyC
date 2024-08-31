use std::{cell::RefCell, rc::Rc};

use rustyc_hir::Hir;
use rustyc_hir_walker::HirWalker;
use rustyc_mir::Mir;
use rustyc_ty::TyContext;

use crate::mir_building_visitor::MirBuildingVisitor;

pub struct MirBuilder {
    ty_context: Rc<RefCell<TyContext>>,
}

impl MirBuilder {
    pub fn new(ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self { ty_context }
    }

    pub fn build(&self, hir: &Hir) -> rustyc_diagnostics::Result<Mir> {
        let walker = HirWalker::new();
        let mut mir_building_visitor = MirBuildingVisitor::new();
        walker.walk(hir, &mut mir_building_visitor)?;

        Ok(mir_building_visitor.finish())
    }
}
