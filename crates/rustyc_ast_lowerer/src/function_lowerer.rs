use std::{cell::RefCell, rc::Rc};

use rustyc_ty::TyContext;

use crate::block_lowerer::BlockLowerer;

pub struct FunctionLowerer {
    function: Rc<rustyc_ast::items::FunctionItem>,
    ty_context: Rc<RefCell<TyContext>>,
}

impl FunctionLowerer {
    pub fn new(
        function: Rc<rustyc_ast::items::FunctionItem>,
        ty_context: Rc<RefCell<TyContext>>,
    ) -> Self {
        Self {
            function,
            ty_context,
        }
    }

    pub fn lower(self) -> Rc<rustyc_hir::items::FunctionItem> {
        Rc::new(rustyc_hir::items::FunctionItem::new(
            self.function.get_name().to_owned(),
            self.function.get_parameters().to_vec(),
            self.lower_block(self.function.get_body()),
            self.function.get_local_variables().to_vec(),
        ))
    }

    pub fn lower_block(&self, block: Rc<rustyc_ast::Block>) -> Rc<rustyc_hir::Block> {
        let block_lowerer = BlockLowerer::new(block, Rc::clone(&self.ty_context));
        block_lowerer.lower()
    }
}
