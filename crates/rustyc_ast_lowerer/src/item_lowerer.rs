use std::rc::Rc;

use crate::function_lowerer::FunctionLowerer;

pub struct ItemLowerer {
    item: Rc<rustyc_ast::Item>,
}

impl ItemLowerer {
    pub fn new(item: Rc<rustyc_ast::Item>) -> Self {
        Self { item }
    }

    pub fn lower(self) -> Rc<rustyc_hir::Item> {
        let hir_item_kind = match self.item.get_kind() {
            rustyc_ast::ItemKind::Function(function) => {
                rustyc_hir::ItemKind::Function(Self::lower_function(Rc::clone(function)))
            }
        };

        Rc::new(rustyc_hir::Item::new(
            hir_item_kind,
            self.item.get_span().clone(),
        ))
    }

    fn lower_function(function: Rc<rustyc_ast::FunctionItem>) -> Rc<rustyc_hir::FunctionItem> {
        let lowerer = FunctionLowerer::new(function);
        lowerer.lower()
    }
}
