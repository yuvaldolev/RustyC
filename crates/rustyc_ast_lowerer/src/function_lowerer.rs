use std::rc::Rc;

use crate::block_lowerer::BlockLowerer;

pub struct FunctionLowerer {
    function: Rc<rustyc_ast::FunctionItem>,
}

impl FunctionLowerer {
    pub fn new(function: Rc<rustyc_ast::FunctionItem>) -> Self {
        Self { function }
    }

    pub fn lower(self) -> Rc<rustyc_hir::FunctionItem> {
        Rc::new(rustyc_hir::FunctionItem::new(
            self.function.get_name().to_owned(),
            self.function.get_parameters().to_vec(),
            Self::lower_block(self.function.get_body()),
            self.function.get_local_variables().to_vec(),
        ))
    }

    pub fn lower_block(block: Rc<rustyc_ast::Block>) -> Rc<rustyc_hir::Block> {
        let block_lowerer = BlockLowerer::new(block);
        block_lowerer.lower()
    }
}
