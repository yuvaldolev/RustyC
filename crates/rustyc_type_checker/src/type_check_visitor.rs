use std::{cell::RefCell, rc::Rc};

use rustyc_diagnostics::Diagnostic;
use rustyc_hir::expressions::{BinaryOperator, Expression};
use rustyc_hir_walker::HirVisitor;
use rustyc_span::Span;
use rustyc_ty::{TyContext, TyMatcher};

pub struct TypeCheckVisitor {
    ty_matcher: TyMatcher,
    span_stack: Vec<Span>,
}

impl TypeCheckVisitor {
    pub fn new(ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self {
            ty_matcher: TyMatcher::new(ty_context),
            span_stack: Vec::new(),
        }
    }

    fn check_add(&self, left: &Expression, right: &Expression) -> rustyc_diagnostics::Result<()> {
        if self.ty_matcher.is_int(left.get_ty()) && self.ty_matcher.is_int(right.get_ty()) {
            return Ok(());
        }

        if self.ty_matcher.is_pointer(left.get_ty()) && self.ty_matcher.is_int(right.get_ty()) {
            return Ok(());
        }

        Err(Diagnostic::new_error(
            rustyc_diagnostics::Error::InvalidBinaryExpressionOperands,
            self.span_stack.last().unwrap().clone(),
        ))
    }

    fn check_subtract(
        &self,
        left: &Expression,
        right: &Expression,
    ) -> rustyc_diagnostics::Result<()> {
        if self.ty_matcher.is_int(left.get_ty()) && self.ty_matcher.is_int(right.get_ty()) {
            return Ok(());
        }

        if self.ty_matcher.is_pointer(left.get_ty()) && self.ty_matcher.is_int(right.get_ty()) {
            return Ok(());
        }

        if self.ty_matcher.is_pointer(left.get_ty()) && self.ty_matcher.is_pointer(right.get_ty()) {
            return Ok(());
        }

        Err(Diagnostic::new_error(
            rustyc_diagnostics::Error::InvalidBinaryExpressionOperands,
            self.span_stack.last().unwrap().clone(),
        ))
    }
}

impl HirVisitor for TypeCheckVisitor {
    fn visit_expression(
        &mut self,
        expression: &Expression,
        walker: &rustyc_hir_walker::HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        self.span_stack.push(expression.get_span().clone());
        walker.walk_expression(expression, self)?;
        self.span_stack.pop();

        Ok(())
    }

    fn visit_binary_expression(
        &mut self,
        expression: &rustyc_hir::expressions::BinaryExpression,
        walker: &rustyc_hir_walker::HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_binary_expression(expression, self)?;

        match expression.get_operator() {
            BinaryOperator::Add => {
                self.check_add(&expression.get_left(), &expression.get_right())?
            }
            BinaryOperator::Subtract => {
                self.check_subtract(&expression.get_left(), &expression.get_right())?
            }
            _ => {}
        }

        Ok(())
    }
}
