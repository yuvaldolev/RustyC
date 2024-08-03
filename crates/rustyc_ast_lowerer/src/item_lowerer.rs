use std::rc::Rc;

use rustyc_ty::TyContext;

use crate::function_lowerer::FunctionLowerer;

pub struct ItemLowerer {
    item: Rc<rustyc_ast::Item>,
    ty_context: Rc<TyContext>,
}

impl ItemLowerer {
    pub fn new(item: Rc<rustyc_ast::Item>, ty_context: Rc<TyContext>) -> Self {
        Self { item, ty_context }
    }

    pub fn lower(self) -> Rc<rustyc_hir::Item> {
        let hir_item_kind = match self.item.get_kind() {
            rustyc_ast::ItemKind::Function(function) => {
                rustyc_hir::ItemKind::Function(self.lower_function(Rc::clone(function)))
            }
        };

        Rc::new(rustyc_hir::Item::new(
            hir_item_kind,
            self.item.get_span().clone(),
        ))
    }

    fn lower_function(
        &self,
        function: Rc<rustyc_ast::FunctionItem>,
    ) -> Rc<rustyc_hir::FunctionItem> {
        let lowerer = FunctionLowerer::new(function, Rc::clone(&self.ty_context));
        lowerer.lower()
    }
}
