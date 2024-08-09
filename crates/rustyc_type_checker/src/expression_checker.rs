use std::{cell::RefCell, rc::Rc};

use rustyc_diagnostics::Diagnostic;
use rustyc_hir::expressions::{
    AssignmentExpression, BinaryExpression, BinaryOperator, Expression, ExpressionKind,
    FunctionCallExpression, UnaryExpression,
};
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
            ExpressionKind::Assignment(expression) => self.check_assignment_expression(expression),
            ExpressionKind::Binary(expression) => self.check_binary_expression(expression),
            ExpressionKind::Unary(expression) => self.check_unary_expression(expression),
            ExpressionKind::FunctionCall(expression) => {
                self.check_function_call_expression(expression)
            }
            _ => Ok(()),
        }
    }

    fn check_assignment_expression(
        &self,
        expression: &AssignmentExpression,
    ) -> rustyc_diagnostics::Result<()> {
        self.check_expression(expression.get_left())?;
        self.check_expression(expression.get_right())?;

        Ok(())
    }

    fn check_binary_expression(
        &self,
        expression: &BinaryExpression,
    ) -> rustyc_diagnostics::Result<()> {
        self.check_expression(expression.get_left())?;
        self.check_expression(expression.get_right())?;

        match expression.get_operator() {
            BinaryOperator::Add => self.check_add(expression.get_left(), expression.get_right()),
            BinaryOperator::Subtract => {
                self.check_subtract(expression.get_left(), expression.get_right())
            }
            _ => Ok(()),
        }
    }

    fn check_unary_expression(
        &self,
        expression: &UnaryExpression,
    ) -> rustyc_diagnostics::Result<()> {
        self.check_expression(expression.get_operand())
    }

    fn check_function_call_expression(
        &self,
        expression: &FunctionCallExpression,
    ) -> rustyc_diagnostics::Result<()> {
        for argument in expression.get_arguments().iter() {
            self.check_expression(Rc::clone(argument))?;
        }

        Ok(())
    }

    fn check_add(
        &self,
        left: Rc<Expression>,
        right: Rc<Expression>,
    ) -> rustyc_diagnostics::Result<()> {
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
        left: Rc<Expression>,
        right: Rc<Expression>,
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
