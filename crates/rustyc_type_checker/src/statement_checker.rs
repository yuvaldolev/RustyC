use std::{cell::RefCell, rc::Rc};

use rustyc_hir::{
    expressions::Expression,
    statements::{
        CompoundStatement, ExpressionStatement, IfStatement, LoopStatement, ReturnStatement,
        Statement, StatementKind,
    },
    Block,
};
use rustyc_ty::TyContext;

use crate::{block_checker::BlockChecker, expression_checker::ExpressionChecker};

pub struct StatementChecker {
    statement: Rc<Statement>,
    ty_context: Rc<RefCell<TyContext>>,
}

impl StatementChecker {
    pub fn new(statement: Rc<Statement>, ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self {
            statement,
            ty_context,
        }
    }

    pub fn check(self) -> rustyc_diagnostics::Result<()> {
        match self.statement.get_kind() {
            StatementKind::Return(statement) => self.check_return_statement(statement),
            StatementKind::If(statement) => self.check_if_statement(statement),
            StatementKind::Loop(statement) => self.check_loop_statement(statement),
            StatementKind::Compound(statement) => self.check_compound_statement(statement),
            StatementKind::Expression(statement) => self.check_expression_statement(statement),
        }
    }

    fn check_return_statement(
        &self,
        statement: &ReturnStatement,
    ) -> rustyc_diagnostics::Result<()> {
        self.check_expression(statement.get_expression())
    }

    fn check_if_statement(&self, statement: &IfStatement) -> rustyc_diagnostics::Result<()> {
        self.check_expression(statement.get_condition_expression())?;
        self.check_statement(statement.get_then_statement())?;
        statement
            .get_else_statement()
            .map(|statement| self.check_statement(statement))
            .transpose()?;

        Ok(())
    }

    fn check_loop_statement(&self, statement: &LoopStatement) -> rustyc_diagnostics::Result<()> {
        statement
            .get_initialization_statement()
            .map(|statement| self.check_statement(statement))
            .transpose()?;
        statement
            .get_condition_expression()
            .map(|expression| self.check_expression(expression))
            .transpose()?;
        statement
            .get_incrementation_expression()
            .map(|expression| self.check_expression(expression))
            .transpose()?;
        self.check_statement(statement.get_then_statement())?;

        Ok(())
    }

    fn check_compound_statement(
        &self,
        statement: &CompoundStatement,
    ) -> rustyc_diagnostics::Result<()> {
        self.check_block(statement.get_block())
    }

    fn check_expression_statement(
        &self,
        statement: &ExpressionStatement,
    ) -> rustyc_diagnostics::Result<()> {
        self.check_expression(statement.get_expression())
    }

    fn check_statement(&self, statement: Rc<Statement>) -> rustyc_diagnostics::Result<()> {
        let statement_checker = Self::new(statement, Rc::clone(&self.ty_context));
        statement_checker.check()
    }

    fn check_expression(&self, expression: Rc<Expression>) -> rustyc_diagnostics::Result<()> {
        let expression_checker = ExpressionChecker::new(expression, Rc::clone(&self.ty_context));
        expression_checker.check()
    }

    fn check_block(&self, block: Rc<Block>) -> rustyc_diagnostics::Result<()> {
        let block_checker = BlockChecker::new(block, Rc::clone(&self.ty_context));
        block_checker.check()
    }
}
