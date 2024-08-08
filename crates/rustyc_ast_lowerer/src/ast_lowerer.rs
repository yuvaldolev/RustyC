use std::{cell::RefCell, rc::Rc};

use rustyc_ty::TyContext;

use crate::item_lowerer::ItemLowerer;

pub struct AstLowerer {
    ast: Vec<Rc<rustyc_ast::items::Item>>,
    ty_context: Rc<RefCell<TyContext>>,
}

impl AstLowerer {
    pub fn new(ast: Vec<Rc<rustyc_ast::items::Item>>, ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self { ast, ty_context }
    }

    pub fn lower(self) -> Rc<Vec<Rc<rustyc_hir::items::Item>>> {
        Rc::new(
            self.ast
                .iter()
                .map(|item| self.lower_item(Rc::clone(item)))
                .collect(),
        )
    }

    fn lower_item(&self, item: Rc<rustyc_ast::items::Item>) -> Rc<rustyc_hir::items::Item> {
        let item_lowerer = ItemLowerer::new(item, Rc::clone(&self.ty_context));
        item_lowerer.lower()
    }
}
