use std::rc::Rc;

use crate::{block_lowerer::BlockLowerer, expression_lowerer::ExpressionLowerer};

pub struct StatementLowerer {
    statement: Rc<rustyc_ast::Statement>,
}

impl StatementLowerer {
    pub fn new(statement: Rc<rustyc_ast::Statement>) -> Self {
        Self { statement }
    }

    pub fn lower(self) -> Rc<rustyc_hir::Statement> {
        let hir_statement_kind = match self.statement.get_kind() {
            rustyc_ast::StatementKind::Return(expression) => {
                rustyc_hir::StatementKind::Return(Self::lower_expression(Rc::clone(expression)))
            }
            rustyc_ast::StatementKind::If(condition_expression, then_statement, else_statement) => {
                rustyc_hir::StatementKind::If(
                    Self::lower_expression(Rc::clone(condition_expression)),
                    Self::lower_statement(Rc::clone(then_statement)),
                    else_statement
                        .as_ref()
                        .map(|value| Self::lower_statement(Rc::clone(value))),
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
                    .map(|value| Self::lower_statement(Rc::clone(value))),
                condition_expression
                    .as_ref()
                    .map(|value| Self::lower_expression(Rc::clone(value))),
                incrementation_expression
                    .as_ref()
                    .map(|value| Self::lower_expression(Rc::clone(value))),
                Self::lower_statement(Rc::clone(then_statement)),
            ),
            rustyc_ast::StatementKind::Compound(block) => {
                rustyc_hir::StatementKind::Compound(Self::lower_block(Rc::clone(block)))
            }
            rustyc_ast::StatementKind::Expression(expression) => {
                rustyc_hir::StatementKind::Expression(Self::lower_expression(Rc::clone(expression)))
            }
        };

        Rc::new(rustyc_hir::Statement::new(
            hir_statement_kind,
            self.statement.get_span().clone(),
        ))
    }

    pub fn lower_statement(statement: Rc<rustyc_ast::Statement>) -> Rc<rustyc_hir::Statement> {
        let statement_lowerer = Self::new(statement);
        statement_lowerer.lower()
    }

    pub fn lower_expression(expression: Rc<rustyc_ast::Expression>) -> Rc<rustyc_hir::Expression> {
        let expression_lowerer = ExpressionLowerer::new(expression);
        expression_lowerer.lower()
    }

    pub fn lower_block(block: Rc<rustyc_ast::Block>) -> Rc<rustyc_hir::Block> {
        let block_lowerer = BlockLowerer::new(block);
        block_lowerer.lower()
    }
}
