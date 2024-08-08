use std::{cell::RefCell, rc::Rc};

use rustyc_hir::{Block, Expression, Statement, StatementKind};
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
            StatementKind::Return(expression) => self.check_expression(Rc::clone(expression)),
            StatementKind::If(condition_expression, then_statement, else_statement) => self
                .check_if(
                    Rc::clone(condition_expression),
                    Rc::clone(then_statement),
                    else_statement.as_ref().map(|value| Rc::clone(value)),
                ),
            StatementKind::Loop(
                initialization_statement,
                condition_expression,
                incrementation_expression,
                then_statement,
            ) => self.check_loop(
                initialization_statement
                    .as_ref()
                    .map(|value| Rc::clone(value)),
                condition_expression.as_ref().map(|value| Rc::clone(value)),
                incrementation_expression
                    .as_ref()
                    .map(|value| Rc::clone(value)),
                Rc::clone(then_statement),
            ),
            StatementKind::Compound(block) => self.check_block(Rc::clone(block)),
            StatementKind::Expression(expression) => self.check_expression(Rc::clone(expression)),
        }
    }

    fn check_if(
        &self,
        condition_expression: Rc<Expression>,
        then_statement: Rc<Statement>,
        else_statement: Option<Rc<Statement>>,
    ) -> rustyc_diagnostics::Result<()> {
        self.check_expression(condition_expression)?;
        self.check_statement(then_statement)?;
        else_statement
            .map(|statement| self.check_statement(statement))
            .transpose()?;

        Ok(())
    }

    fn check_loop(
        &self,
        initialization_statement: Option<Rc<Statement>>,
        condition_expression: Option<Rc<Expression>>,
        incrementation_expression: Option<Rc<Expression>>,
        then_statement: Rc<Statement>,
    ) -> rustyc_diagnostics::Result<()> {
        initialization_statement
            .map(|statement| self.check_statement(statement))
            .transpose()?;
        condition_expression
            .map(|expression| self.check_expression(expression))
            .transpose()?;
        incrementation_expression
            .map(|expression| self.check_expression(expression))
            .transpose()?;
        self.check_statement(then_statement)?;

        Ok(())
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
