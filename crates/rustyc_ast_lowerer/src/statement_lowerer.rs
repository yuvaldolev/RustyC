use std::{cell::RefCell, rc::Rc};

use rustyc_ty::TyContext;

use crate::{block_lowerer::BlockLowerer, expression_lowerer::ExpressionLowerer};

pub struct StatementLowerer {
    statement: Rc<rustyc_ast::Statement>,
    ty_context: Rc<RefCell<TyContext>>,
}

impl StatementLowerer {
    pub fn new(statement: Rc<rustyc_ast::Statement>, ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self {
            statement,
            ty_context,
        }
    }

    pub fn lower(self) -> Rc<rustyc_hir::Statement> {
        let hir_statement_kind = match self.statement.get_kind() {
            rustyc_ast::StatementKind::Return(expression) => {
                rustyc_hir::StatementKind::Return(self.lower_expression(Rc::clone(expression)))
            }
            rustyc_ast::StatementKind::If(condition_expression, then_statement, else_statement) => {
                rustyc_hir::StatementKind::If(
                    self.lower_expression(Rc::clone(condition_expression)),
                    self.lower_statement(Rc::clone(then_statement)),
                    else_statement
                        .as_ref()
                        .map(|value| self.lower_statement(Rc::clone(value))),
                )
            }
            rustyc_ast::StatementKind::Loop(
                initialization_statement,
                condition_expression,
                incrementation_expression,
                then_statement,
            ) => rustyc_hir::StatementKind::Loop(
                initialization_statement
                    .as_ref()
                    .map(|value| self.lower_statement(Rc::clone(value))),
                condition_expression
                    .as_ref()
                    .map(|value| self.lower_expression(Rc::clone(value))),
                incrementation_expression
                    .as_ref()
                    .map(|value| self.lower_expression(Rc::clone(value))),
                self.lower_statement(Rc::clone(then_statement)),
            ),
            rustyc_ast::StatementKind::Compound(block) => {
                rustyc_hir::StatementKind::Compound(self.lower_block(Rc::clone(block)))
            }
            rustyc_ast::StatementKind::Expression(expression) => {
                rustyc_hir::StatementKind::Expression(self.lower_expression(Rc::clone(expression)))
            }
        };

        Rc::new(rustyc_hir::Statement::new(
            hir_statement_kind,
            self.statement.get_span().clone(),
        ))
    }

    fn lower_statement(&self, statement: Rc<rustyc_ast::Statement>) -> Rc<rustyc_hir::Statement> {
        let statement_lowerer = Self::new(statement, Rc::clone(&self.ty_context));
        statement_lowerer.lower()
    }

    fn lower_expression(
        &self,
        expression: Rc<rustyc_ast::Expression>,
    ) -> Rc<rustyc_hir::Expression> {
        let expression_lowerer = ExpressionLowerer::new(expression, Rc::clone(&self.ty_context));
        expression_lowerer.lower()
    }

    fn lower_block(&self, block: Rc<rustyc_ast::Block>) -> Rc<rustyc_hir::Block> {
        let block_lowerer = BlockLowerer::new(block, Rc::clone(&self.ty_context));
        block_lowerer.lower()
    }
}
