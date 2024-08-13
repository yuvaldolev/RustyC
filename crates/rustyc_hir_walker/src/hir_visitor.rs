use rustyc_hir::{
    items::{FunctionItem, Item},
    statements::{ExpressionStatement, IfStatement, LoopStatement, ReturnStatement, Statement},
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

    fn visit_statement(
        &mut self,
        statement: &Statement,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_statement(statement, self)
    }

    fn visit_return_statement(
        &mut self,
        statement: &ReturnStatement,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_return_statement(statement, self)
    }

    fn visit_if_statement(
        &mut self,
        statement: &IfStatement,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_if_statement(statement, self)
    }

    fn visit_loop_statement(
        &mut self,
        statement: &LoopStatement,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_loop_statement(statement, self)
    }

    fn visit_compound_statement(
        &mut self,
        statement: &CompoundStatement,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_compound_statement(statement, self)
    }

    fn visit_expression_statement(
        &mut self,
        statement: &ExpressionStatement,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_expression_statement(statement, self)
    }
}
