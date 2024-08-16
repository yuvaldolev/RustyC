use rustyc_hir::{
    expressions::{
        AssignmentExpression, BinaryExpression, Expression, ExpressionKind, FunctionCallExpression,
        UnaryExpression,
    },
    items::{FunctionItem, Item, ItemKind},
    statements::{
        CompoundStatement, ExpressionStatement, IfStatement, LoopStatement, ReturnStatement,
        Statement, StatementKind,
    },
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

    pub fn walk_statement(
        &self,
        statement: &Statement,
        visitor: &mut impl HirVisitor,
    ) -> rustyc_diagnostics::Result<()> {
        match statement.get_kind() {
            StatementKind::Return(statement) => visitor.visit_return_statement(statement, self),
            StatementKind::If(statement) => visitor.visit_if_statement(statement, self),
            StatementKind::Loop(statement) => visitor.visit_loop_statement(statement, self),
            StatementKind::Compound(statement) => visitor.visit_compound_statement(statement, self),
            StatementKind::Expression(statement) => {
                visitor.visit_expression_statement(statement, self)
            }
        }
    }

    pub fn walk_return_statement(
        &self,
        statement: &ReturnStatement,
        visitor: &mut impl HirVisitor,
    ) -> rustyc_diagnostics::Result<()> {
        visitor.visit_expression(&statement.get_expression(), self)
    }

    pub fn walk_if_statement(
        &self,
        statement: &IfStatement,
        visitor: &mut impl HirVisitor,
    ) -> rustyc_diagnostics::Result<()> {
        visitor.visit_expression(&statement.get_condition_expression(), self)?;

        visitor.visit_statement(&statement.get_then_statement(), self)?;

        if let Some(else_statement) = statement.get_else_statement() {
            visitor.visit_statement(&else_statement, self)?;
        }

        Ok(())
    }

    pub fn walk_loop_statement(
        &self,
        statement: &LoopStatement,
        visitor: &mut impl HirVisitor,
    ) -> rustyc_diagnostics::Result<()> {
        if let Some(initialization_statement) = statement.get_initialization_statement() {
            visitor.visit_statement(&initialization_statement, self)?;
        }

        if let Some(condition_expression) = statement.get_condition_expression() {
            visitor.visit_expression(&condition_expression, self)?;
        }

        if let Some(incrementation_expression) = statement.get_incrementation_expression() {
            visitor.visit_expression(&incrementation_expression, self)?;
        }

        visitor.visit_statement(&statement.get_then_statement(), self)?;

        Ok(())
    }

    pub fn walk_compound_statement(
        &self,
        statement: &CompoundStatement,
        visitor: &mut impl HirVisitor,
    ) -> rustyc_diagnostics::Result<()> {
        visitor.visit_block(&statement.get_block(), self)
    }

    pub fn walk_expression_statement(
        &self,
        statement: &ExpressionStatement,
        visitor: &mut impl HirVisitor,
    ) -> rustyc_diagnostics::Result<()> {
        visitor.visit_expression(&statement.get_expression(), self)
    }

    pub fn walk_expression(
        &self,
        expression: &Expression,
        visitor: &mut impl HirVisitor,
    ) -> rustyc_diagnostics::Result<()> {
        match expression.get_kind() {
            ExpressionKind::Assignment(expression) => {
                visitor.visit_assignment_expression(expression, self)
            }
            ExpressionKind::Binary(expression) => visitor.visit_binary_expression(expression, self),
            ExpressionKind::Unary(expression) => visitor.visit_unary_expression(expression, self),
            ExpressionKind::FunctionCall(expression) => {
                visitor.visit_function_call_expression(expression, self)
            }
            _ => Ok(()),
        }
    }

    pub fn walk_assignment_expression(
        &self,
        expression: &AssignmentExpression,
        visitor: &mut impl HirVisitor,
    ) -> rustyc_diagnostics::Result<()> {
        visitor.visit_expression(&expression.get_left(), self)?;
        visitor.visit_expression(&expression.get_right(), self)?;

        Ok(())
    }

    pub fn walk_binary_expression(
        &self,
        expression: &BinaryExpression,
        visitor: &mut impl HirVisitor,
    ) -> rustyc_diagnostics::Result<()> {
        visitor.visit_expression(&expression.get_left(), self)?;
        visitor.visit_expression(&expression.get_right(), self)?;

        Ok(())
    }

    pub fn walk_unary_expression(
        &self,
        expression: &UnaryExpression,
        visitor: &mut impl HirVisitor,
    ) -> rustyc_diagnostics::Result<()> {
        visitor.visit_expression(&expression.get_operand(), self)
    }

    pub fn walk_function_call_expression(
        &self,
        expression: &FunctionCallExpression,
        visitor: &mut impl HirVisitor,
    ) -> rustyc_diagnostics::Result<()> {
        for argument in expression.get_arguments().iter() {
            visitor.visit_expression(argument, self)?;
        }

        Ok(())
    }
}
