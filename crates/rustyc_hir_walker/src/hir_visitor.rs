use rustyc_hir::{
    items::{FunctionItem, Item},
    Block,
};

use crate::HirWalker;

pub trait HirVisitor: Sized {
    fn visit_item(&mut self, item: &Item, walker: &HirWalker) -> rustyc_diagnostics::Result<()> {
        walker.walk_item(item, self)
    }

    fn visit_function(
        &mut self,
        function: &FunctionItem,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_function(function, self)
    }

    fn visit_block(&mut self, block: &Block, walker: &HirWalker) -> rustyc_diagnostics::Result<()> {
        walker.walk_block(block, self)
    }
}
