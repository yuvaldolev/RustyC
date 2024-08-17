use rustyc_hir::{
    expressions::{
        AssignmentExpression, BinaryExpression, Expression, FunctionCallExpression,
        NumberExpression, UnaryExpression, VariableExpression,
    },
    items::{FunctionItem, Item},
    statements::{
        CompoundStatement, ExpressionStatement, IfStatement, LoopStatement, ReturnStatement,
        Statement,
    },
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

    fn visit_expression(
        &mut self,
        expression: &Expression,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_expression(expression, self)
    }

    fn visit_assignment_expression(
        &mut self,
        expression: &AssignmentExpression,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_assignment_expression(expression, self)
    }

    fn visit_binary_expression(
        &mut self,
        expression: &BinaryExpression,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_binary_expression(expression, self)
    }

    fn visit_unary_expression(
        &mut self,
        expression: &UnaryExpression,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_unary_expression(expression, self)
    }

    fn visit_variable_expression(
        &mut self,
        expression: &VariableExpression,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_variable_expression(expression, self)
    }

    fn visit_number_expression(
        &mut self,
        expression: &NumberExpression,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_number_expression(expression, self)
    }

    fn visit_function_call_expression(
        &mut self,
        expression: &FunctionCallExpression,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_function_call_expression(expression, self)
    }
}
