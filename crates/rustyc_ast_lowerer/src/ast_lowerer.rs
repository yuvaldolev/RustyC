use std::{cell::RefCell, rc::Rc};

use rustyc_ast::Ast;
use rustyc_hir::Hir;
use rustyc_ty::TyContext;

use crate::item_lowerer::ItemLowerer;

pub struct AstLowerer {
    ast: Ast,
    ty_context: Rc<RefCell<TyContext>>,
}

impl AstLowerer {
    pub fn new(ast: Ast, ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self { ast, ty_context }
    }

    pub fn lower(self) -> rustyc_diagnostics::Result<Rc<Hir>> {
        // TODO: Can we get rid of `Rc` using `into_iter` instead of `iter`?
        Ok(Rc::new(Hir::new(
            self.ast
                .get_items()
                .iter()
                .map(|item| self.lower_item(Rc::clone(item)))
                .collect::<rustyc_diagnostics::Result<Vec<_>>>()?,
        )))
    }

    fn lower_item(
        &self,
        item: Rc<rustyc_ast::items::Item>,
    ) -> rustyc_diagnostics::Result<Rc<rustyc_hir::items::Item>> {
        let item_lowerer = ItemLowerer::new(item, Rc::clone(&self.ty_context));
        item_lowerer.lower()
    }
}
