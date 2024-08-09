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

    pub fn lower(self) -> Rc<Hir> {
        Rc::new(Hir::new(
            self.ast
                .get_items()
                .iter()
                .map(|item| self.lower_item(Rc::clone(item)))
                .collect(),
        ))
    }

    fn lower_item(&self, item: Rc<rustyc_ast::items::Item>) -> Rc<rustyc_hir::items::Item> {
        let item_lowerer = ItemLowerer::new(item, Rc::clone(&self.ty_context));
        item_lowerer.lower()
    }
}
