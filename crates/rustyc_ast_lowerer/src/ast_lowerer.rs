use std::rc::Rc;

use crate::item_lowerer::ItemLowerer;

pub struct AstLowerer {
    ast: Vec<Rc<rustyc_ast::Item>>,
}

impl AstLowerer {
    pub fn new(ast: Vec<Rc<rustyc_ast::Item>>) -> Self {
        Self { ast }
    }

    pub fn lower(self) -> Vec<Rc<rustyc_hir::Item>> {
        self.ast
            .iter()
            .map(|item| Self::lower_item(Rc::clone(item)))
            .collect()
    }

    fn lower_item(item: Rc<rustyc_ast::Item>) -> Rc<rustyc_hir::Item> {
        let item_lowerer = ItemLowerer::new(item);
        item_lowerer.lower()
    }
}
