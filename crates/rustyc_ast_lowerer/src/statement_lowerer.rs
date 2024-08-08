use std::{cell::RefCell, rc::Rc};

use rustyc_ty::TyContext;

use crate::{block_lowerer::BlockLowerer, expression_lowerer::ExpressionLowerer};

pub struct StatementLowerer {
    statement: Rc<rustyc_ast::statements::Statement>,
    ty_context: Rc<RefCell<TyContext>>,
}

impl StatementLowerer {
    pub fn new(
        statement: Rc<rustyc_ast::statements::Statement>,
        ty_context: Rc<RefCell<TyContext>>,
    ) -> Self {
        Self {
            statement,
            ty_context,
        }
    }

    pub fn lower(self) -> Rc<rustyc_hir::statements::Statement> {
        let hir_statement_kind = match self.statement.get_kind() {
            rustyc_ast::statements::StatementKind::Return(statement) => {
                rustyc_hir::statements::StatementKind::Return(
                    self.lower_return_statement(statement),
                )
            }
            rustyc_ast::statements::StatementKind::If(statement) => {
                rustyc_hir::statements::StatementKind::If(self.lower_if_statement(statement))
            }
            rustyc_ast::statements::StatementKind::Loop(statement) => {
                rustyc_hir::statements::StatementKind::Loop(self.lower_loop_statement(statement))
            }
            rustyc_ast::statements::StatementKind::Compound(statement) => {
                rustyc_hir::statements::StatementKind::Compound(
                    self.lower_compound_statement(statement),
                )
            }
            rustyc_ast::statements::StatementKind::Expression(expression) => {
                rustyc_hir::statements::StatementKind::Expression(
                    self.lower_expression_statement(expression),
                )
            }
        };

        Rc::new(rustyc_hir::statements::Statement::new(
            hir_statement_kind,
            self.statement.get_span().clone(),
        ))
    }

    fn lower_return_statement(
        &self,
        statement: &rustyc_ast::statements::ReturnStatement,
    ) -> rustyc_hir::statements::ReturnStatement {
        rustyc_hir::statements::ReturnStatement::new(
            self.lower_expression(statement.get_expression()),
        )
    }

    fn lower_if_statement(
        &self,
        statement: &rustyc_ast::statements::IfStatement,
    ) -> rustyc_hir::statements::IfStatement {
        rustyc_hir::statements::IfStatement::new(
            self.lower_expression(statement.get_condition_expression()),
            self.lower_statement(statement.get_then_statement()),
            statement
                .get_else_statement()
                .map(|value| self.lower_statement(value)),
        )
    }

    fn lower_loop_statement(
        &self,
        statement: &rustyc_ast::statements::LoopStatement,
    ) -> rustyc_hir::statements::LoopStatement {
        rustyc_hir::statements::LoopStatement::new(
            statement
                .get_initialization_statement()
                .map(|value| self.lower_statement(value)),
            statement
                .get_condition_expression()
                .map(|value| self.lower_expression(value)),
            statement
                .get_incrementation_expression()
                .map(|value| self.lower_expression(value)),
            self.lower_statement(statement.get_then_statement()),
        )
    }

    fn lower_compound_statement(
        &self,
        statement: &rustyc_ast::statements::CompoundStatement,
    ) -> rustyc_hir::statements::CompoundStatement {
        rustyc_hir::statements::CompoundStatement::new(self.lower_block(statement.get_block()))
    }

    fn lower_expression_statement(
        &self,
        statement: &rustyc_ast::statements::ExpressionStatement,
    ) -> rustyc_hir::statements::ExpressionStatement {
        rustyc_hir::statements::ExpressionStatement::new(
            self.lower_expression(statement.get_expression()),
        )
    }

    fn lower_statement(
        &self,
        statement: Rc<rustyc_ast::statements::Statement>,
    ) -> Rc<rustyc_hir::statements::Statement> {
        let statement_lowerer = Self::new(statement, Rc::clone(&self.ty_context));
        statement_lowerer.lower()
    }

    fn lower_expression(
        &self,
        expression: Rc<rustyc_ast::expressions::Expression>,
    ) -> Rc<rustyc_hir::expressions::Expression> {
        let expression_lowerer = ExpressionLowerer::new(expression, Rc::clone(&self.ty_context));
        expression_lowerer.lower()
    }

    fn lower_block(&self, block: Rc<rustyc_ast::Block>) -> Rc<rustyc_hir::Block> {
        let block_lowerer = BlockLowerer::new(block, Rc::clone(&self.ty_context));
        block_lowerer.lower()
    }
}
