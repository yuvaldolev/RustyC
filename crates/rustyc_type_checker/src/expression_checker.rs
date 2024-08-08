use std::{cell::RefCell, rc::Rc};

use rustyc_diagnostics::Diagnostic;
use rustyc_hir::{BinaryOperator, Expression, ExpressionKind};
use rustyc_ty::{TyContext, TyMatcher};

pub struct ExpressionChecker {
    expression: Rc<Expression>,
    ty_matcher: TyMatcher,
    ty_context: Rc<RefCell<TyContext>>,
}

impl ExpressionChecker {
    pub fn new(expression: Rc<Expression>, ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self {
            expression,
            ty_matcher: TyMatcher::new(Rc::clone(&ty_context)),
            ty_context,
        }
    }

    pub fn check(self) -> rustyc_diagnostics::Result<()> {
        match self.expression.get_kind() {
            ExpressionKind::Assignment(left, right) => {
                self.check_assignment(Rc::clone(left), Rc::clone(right))
            }
            ExpressionKind::Binary(operator, left, right) => {
                self.check_binary(operator, Rc::clone(left), Rc::clone(right))
            }
            ExpressionKind::Unary(_, right) => self.check_expression(Rc::clone(right)),
            ExpressionKind::FunctionCall(_, arguments) => self.check_function_call(arguments),
            _ => Ok(()),
        }
    }

    fn check_assignment(
        &self,
        left: Rc<Expression>,
        right: Rc<Expression>,
    ) -> rustyc_diagnostics::Result<()> {
        self.check_expression(left)?;
        self.check_expression(right)?;

        Ok(())
    }

    fn check_binary(
        &self,
        operator: &BinaryOperator,
        left: Rc<Expression>,
        right: Rc<Expression>,
    ) -> rustyc_diagnostics::Result<()> {
        self.check_expression(Rc::clone(&left))?;
        self.check_expression(Rc::clone(&right))?;

        match operator {
            BinaryOperator::Add => self.check_add(&left, &right),
            BinaryOperator::Subtract => self.check_subtract(&left, &right),
            _ => Ok(()),
        }
    }

    fn check_function_call(&self, arguments: &[Rc<Expression>]) -> rustyc_diagnostics::Result<()> {
        for argument in arguments.iter() {
            self.check_expression(Rc::clone(argument))?;
        }

        Ok(())
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
            self.expression.get_span().clone(),
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
            self.expression.get_span().clone(),
        ))
    }

    fn check_expression(&self, expression: Rc<Expression>) -> rustyc_diagnostics::Result<()> {
        let expression_checker = Self::new(expression, Rc::clone(&self.ty_context));
        expression_checker.check()
    }
}
