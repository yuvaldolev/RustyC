use rustyc_hir::{
    items::{FunctionItem, Item, ItemKind},
    Block, Hir,
};

use crate::HirVisitor;

pub struct HirWalker {
    hir: Hir,
}

impl HirWalker {
    pub fn new(hir: Hir) -> Self {
        Self { hir }
    }

    pub fn walk(self, visitor: &mut impl HirVisitor) -> rustyc_diagnostics::Result<()> {
        for item in self.hir.get_items().iter() {
            visitor.visit_item(item, &self)?;
        }

        Ok(())
    }

    pub fn walk_item(
        &self,
        item: &Item,
        visitor: &mut impl HirVisitor,
    ) -> rustyc_diagnostics::Result<()> {
        match item.get_kind() {
            ItemKind::Function(function) => visitor.visit_function(function, self),
        }
    }

    pub fn walk_function(
        &self,
        function: &FunctionItem,
        visitor: &mut impl HirVisitor,
    ) -> rustyc_diagnostics::Result<()> {
        // TODO: Consider visiting:
        // 1. Name
        // 2. Parameters
        // 3. Local variables
        visitor.visit_block(&function.get_body(), self)
    }

    pub fn walk_block(
        &self,
        block: &Block,
        visitor: &mut impl HirVisitor,
    ) -> rustyc_diagnostics::Result<()> {
        for statement in block.get_statements().iter() {
            visitor.visit_statement(statement, self)?;
        }

        Ok(())
    }
}
