use std::rc::Rc;

use rustyc_ty::TyContext;

use crate::item_lowerer::ItemLowerer;

pub struct AstLowerer {
    ast: Vec<Rc<rustyc_ast::Item>>,
    ty_context: Rc<TyContext>,
}

impl AstLowerer {
    pub fn new(ast: Vec<Rc<rustyc_ast::Item>>, ty_context: Rc<TyContext>) -> Self {
        Self { ast, ty_context }
    }

    pub fn lower(self) -> Vec<Rc<rustyc_hir::Item>> {
        self.ast
            .iter()
            .map(|item| self.lower_item(Rc::clone(item)))
            .collect()
    }

    fn lower_item(&self, item: Rc<rustyc_ast::Item>) -> Rc<rustyc_hir::Item> {
        let item_lowerer = ItemLowerer::new(item, Rc::clone(&self.ty_context));
        item_lowerer.lower()
    }
}
