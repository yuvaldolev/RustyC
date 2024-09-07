use std::{cell::RefCell, rc::Rc};

use rustyc_diagnostics::Diagnostic;
use rustyc_ty::{Ty, TyId, TyMatcher};

use crate::function_lowering_context::FunctionLoweringContext;

pub struct ExpressionLowerer {
    context: Rc<FunctionLoweringContext>,
    expression: Rc<rustyc_ast::expressions::Expression>,
    ty_matcher: TyMatcher,
    ty_context: Rc<RefCell<rustyc_ty::TyContext>>,
}

impl ExpressionLowerer {
    pub fn new(
        context: Rc<FunctionLoweringContext>,
        expression: Rc<rustyc_ast::expressions::Expression>,
        ty_context: Rc<RefCell<rustyc_ty::TyContext>>,
    ) -> Self {
        Self {
            context,
            expression,
            ty_matcher: TyMatcher::new(Rc::clone(&ty_context)),
            ty_context,
        }
    }

    pub fn lower(self) -> rustyc_diagnostics::Result<Rc<rustyc_hir::expressions::Expression>> {
        let (hir_expression_kind, ty) = match self.expression.get_kind() {
            rustyc_ast::expressions::ExpressionKind::Assignment(expression) => {
                self.lower_assignment_expression(expression)?
            }
            rustyc_ast::expressions::ExpressionKind::Binary(expression) => {
                self.lower_binary_expression(expression)?
            }
            rustyc_ast::expressions::ExpressionKind::Unary(expression) => {
                self.lower_unary_expression(expression)?
            }
            rustyc_ast::expressions::ExpressionKind::Variable(expression) => {
                self.lower_variable_expression(expression)?
            }
            rustyc_ast::expressions::ExpressionKind::Number(expression) => {
                self.lower_number_expression(expression)
            }
            rustyc_ast::expressions::ExpressionKind::FunctionCall(expression) => {
                self.lower_function_call_expression(expression)?
            }
        };

        Ok(Rc::new(rustyc_hir::expressions::Expression::new(
            hir_expression_kind,
            ty,
            self.expression.get_span().clone(),
        )))
    }

    fn lower_binary_operator(
        operator: &rustyc_ast::expressions::BinaryOperator,
    ) -> rustyc_hir::expressions::BinaryOperator {
        match operator {
            rustyc_ast::expressions::BinaryOperator::Equal => {
                rustyc_hir::expressions::BinaryOperator::Equal
            }
            rustyc_ast::expressions::BinaryOperator::NotEqual => {
                rustyc_hir::expressions::BinaryOperator::NotEqual
            }
            rustyc_ast::expressions::BinaryOperator::LessThan => {
                rustyc_hir::expressions::BinaryOperator::LessThan
            }
            rustyc_ast::expressions::BinaryOperator::LessThanOrEqual => {
                rustyc_hir::expressions::BinaryOperator::LessThanOrEqual
            }
            rustyc_ast::expressions::BinaryOperator::Add => {
                rustyc_hir::expressions::BinaryOperator::Add
            }
            rustyc_ast::expressions::BinaryOperator::Subtract => {
                rustyc_hir::expressions::BinaryOperator::Subtract
            }
            rustyc_ast::expressions::BinaryOperator::Multiply => {
                rustyc_hir::expressions::BinaryOperator::Multiply
            }
            rustyc_ast::expressions::BinaryOperator::Divide => {
                rustyc_hir::expressions::BinaryOperator::Divide
            }
        }
    }

    fn lower_unary_operator(
        operator: &rustyc_ast::expressions::UnaryOperator,
    ) -> rustyc_hir::expressions::UnaryOperator {
        match operator {
            rustyc_ast::expressions::UnaryOperator::Negate => {
                rustyc_hir::expressions::UnaryOperator::Negate
            }
            rustyc_ast::expressions::UnaryOperator::AddressOf => {
                rustyc_hir::expressions::UnaryOperator::AddressOf
            }
            rustyc_ast::expressions::UnaryOperator::Dereference => {
                rustyc_hir::expressions::UnaryOperator::Dereference
            }
        }
    }

    fn lower_assignment_expression(
        &self,
        expression: &rustyc_ast::expressions::AssignmentExpression,
    ) -> rustyc_diagnostics::Result<(rustyc_hir::expressions::ExpressionKind, TyId)> {
        let hir_left = self.lower_expression(expression.get_left())?;
        let hir_right = self.lower_expression(expression.get_right())?;

        let ty = hir_left.get_ty();

        Ok((
            rustyc_hir::expressions::ExpressionKind::Assignment(
                rustyc_hir::expressions::AssignmentExpression::new(hir_left, hir_right),
            ),
            ty,
        ))
    }

    fn lower_binary_expression(
        &self,
        expression: &rustyc_ast::expressions::BinaryExpression,
    ) -> rustyc_diagnostics::Result<(rustyc_hir::expressions::ExpressionKind, TyId)> {
        let hir_left = self.lower_expression(expression.get_left())?;
        let hir_right = self.lower_expression(expression.get_right())?;

        let ty = hir_left.get_ty();

        let hir_expression = match expression.get_operator() {
            rustyc_ast::expressions::BinaryOperator::Add => self.lower_add(hir_left, hir_right),
            rustyc_ast::expressions::BinaryOperator::Subtract => {
                self.lower_subtract(hir_left, hir_right)
            }
            _ => (
                rustyc_hir::expressions::ExpressionKind::Binary(
                    rustyc_hir::expressions::BinaryExpression::new(
                        Self::lower_binary_operator(expression.get_operator()),
                        hir_left,
                        hir_right,
                    ),
                ),
                ty,
            ),
        };

        Ok(hir_expression)
    }

    fn lower_unary_expression(
        &self,
        expression: &rustyc_ast::expressions::UnaryExpression,
    ) -> rustyc_diagnostics::Result<(rustyc_hir::expressions::ExpressionKind, TyId)> {
        let hir_operand = self.lower_expression(expression.get_operand())?;

        let ty = match expression.get_operator() {
            rustyc_ast::expressions::UnaryOperator::Negate => hir_operand.get_ty(),
            rustyc_ast::expressions::UnaryOperator::AddressOf => self
                .ty_context
                .borrow_mut()
                .register(Ty::Pointer(hir_operand.get_ty())),
            rustyc_ast::expressions::UnaryOperator::Dereference => {
                let int_ty = self.ty_context.borrow_mut().register(Ty::Int);

                if let Ty::Pointer(base) = self.ty_context.borrow().get(hir_operand.get_ty()) {
                    *base
                } else {
                    int_ty
                }
            }
        };

        Ok((
            rustyc_hir::expressions::ExpressionKind::Unary(
                rustyc_hir::expressions::UnaryExpression::new(
                    Self::lower_unary_operator(expression.get_operator()),
                    hir_operand,
                ),
            ),
            ty,
        ))
    }

    fn lower_variable_expression(
        &self,
        expression: &rustyc_ast::expressions::VariableReferenceExpression,
    ) -> rustyc_diagnostics::Result<(rustyc_hir::expressions::ExpressionKind, TyId)> {
        let hir_local = self
            .context
            .get_hir_local(expression.get_name())
            .ok_or_else(|| {
                Diagnostic::new_error(
                    rustyc_diagnostics::Error::UndeclaredIdentifier(
                        expression.get_name().to_owned(),
                    ),
                    self.expression.get_span().clone(),
                )
            })?;

        Ok((
            rustyc_hir::expressions::ExpressionKind::Variable(
                rustyc_hir::expressions::VariableReferenceExpression::new(hir_local),
            ),
            self.ty_context.borrow_mut().register(Ty::Int),
        ))
    }

    fn lower_number_expression(
        &self,
        expression: &rustyc_ast::expressions::NumberExpression,
    ) -> (rustyc_hir::expressions::ExpressionKind, TyId) {
        (
            rustyc_hir::expressions::ExpressionKind::Number(
                rustyc_hir::expressions::NumberExpression::new(expression.get_value()),
            ),
            self.ty_context.borrow_mut().register(Ty::Int),
        )
    }

    fn lower_function_call_expression(
        &self,
        expression: &rustyc_ast::expressions::FunctionCallExpression,
    ) -> rustyc_diagnostics::Result<(rustyc_hir::expressions::ExpressionKind, TyId)> {
        let hir_arguments = expression
            .get_arguments()
            .iter()
            .map(|argument| self.lower_expression(Rc::clone(argument)))
            .collect::<rustyc_diagnostics::Result<Vec<_>>>()?;

        Ok((
            rustyc_hir::expressions::ExpressionKind::FunctionCall(
                rustyc_hir::expressions::FunctionCallExpression::new(
                    expression.get_name().to_owned(),
                    hir_arguments,
                ),
            ),
            self.ty_context.borrow_mut().register(Ty::Int),
        ))
    }

    fn lower_add(
        &self,
        left: Rc<rustyc_hir::expressions::Expression>,
        right: Rc<rustyc_hir::expressions::Expression>,
    ) -> (rustyc_hir::expressions::ExpressionKind, TyId) {
        if self.ty_matcher.is_pointer(left.get_ty()) && self.ty_matcher.is_int(right.get_ty()) {
            return self.lower_pointer_number_arithmetic(
                left,
                right,
                rustyc_hir::expressions::BinaryOperator::Add,
            );
        }

        if self.ty_matcher.is_int(left.get_ty()) && self.ty_matcher.is_pointer(right.get_ty()) {
            return self.lower_pointer_number_arithmetic(
                right,
                left,
                rustyc_hir::expressions::BinaryOperator::Add,
            );
        }

        let ty = left.get_ty();

        (
            rustyc_hir::expressions::ExpressionKind::Binary(
                rustyc_hir::expressions::BinaryExpression::new(
                    rustyc_hir::expressions::BinaryOperator::Add,
                    left,
                    right,
                ),
            ),
            ty,
        )
    }

    fn lower_subtract(
        &self,
        left: Rc<rustyc_hir::expressions::Expression>,
        right: Rc<rustyc_hir::expressions::Expression>,
    ) -> (rustyc_hir::expressions::ExpressionKind, TyId) {
        if self.ty_matcher.is_pointer(left.get_ty()) && self.ty_matcher.is_int(right.get_ty()) {
            return self.lower_pointer_number_arithmetic(
                left,
                right,
                rustyc_hir::expressions::BinaryOperator::Subtract,
            );
        }

        if self.ty_matcher.is_pointer(left.get_ty()) && self.ty_matcher.is_pointer(right.get_ty()) {
            return self.lower_pointer_pointer_subtract(left, right);
        }

        let ty = left.get_ty();

        (
            rustyc_hir::expressions::ExpressionKind::Binary(
                rustyc_hir::expressions::BinaryExpression::new(
                    rustyc_hir::expressions::BinaryOperator::Subtract,
                    left,
                    right,
                ),
            ),
            ty,
        )
    }

    fn lower_pointer_number_arithmetic(
        &self,
        pointer: Rc<rustyc_hir::expressions::Expression>,
        number: Rc<rustyc_hir::expressions::Expression>,
        operator: rustyc_hir::expressions::BinaryOperator,
    ) -> (rustyc_hir::expressions::ExpressionKind, TyId) {
        let pointer_ty = pointer.get_ty();
        let number_ty = number.get_ty();

        let number_span = number.get_span().clone();

        (
            rustyc_hir::expressions::ExpressionKind::Binary(
                rustyc_hir::expressions::BinaryExpression::new(
                    operator,
                    pointer,
                    Rc::new(rustyc_hir::expressions::Expression::new(
                        rustyc_hir::expressions::ExpressionKind::Binary(
                            rustyc_hir::expressions::BinaryExpression::new(
                                rustyc_hir::expressions::BinaryOperator::Multiply,
                                number,
                                Rc::new(rustyc_hir::expressions::Expression::new(
                                    rustyc_hir::expressions::ExpressionKind::Number(
                                        rustyc_hir::expressions::NumberExpression::new(8),
                                    ),
                                    self.ty_context.borrow_mut().register(Ty::Int),
                                    number_span.clone(),
                                )),
                            ),
                        ),
                        number_ty,
                        number_span,
                    )),
                ),
            ),
            pointer_ty,
        )
    }

    fn lower_pointer_pointer_subtract(
        &self,
        left: Rc<rustyc_hir::expressions::Expression>,
        right: Rc<rustyc_hir::expressions::Expression>,
    ) -> (rustyc_hir::expressions::ExpressionKind, TyId) {
        let int_ty = self.ty_context.borrow_mut().register(Ty::Int);

        (
            rustyc_hir::expressions::ExpressionKind::Binary(
                rustyc_hir::expressions::BinaryExpression::new(
                    rustyc_hir::expressions::BinaryOperator::Divide,
                    Rc::new(rustyc_hir::expressions::Expression::new(
                        rustyc_hir::expressions::ExpressionKind::Binary(
                            rustyc_hir::expressions::BinaryExpression::new(
                                rustyc_hir::expressions::BinaryOperator::Subtract,
                                left,
                                right,
                            ),
                        ),
                        int_ty,
                        self.expression.get_span().clone(),
                    )),
                    Rc::new(rustyc_hir::expressions::Expression::new(
                        rustyc_hir::expressions::ExpressionKind::Number(
                            rustyc_hir::expressions::NumberExpression::new(8),
                        ),
                        int_ty,
                        self.expression.get_span().clone(),
                    )),
                ),
            ),
            int_ty,
        )
    }

    fn lower_expression(
        &self,
        expression: Rc<rustyc_ast::expressions::Expression>,
    ) -> rustyc_diagnostics::Result<Rc<rustyc_hir::expressions::Expression>> {
        let expression_lowerer = Self::new(
            Rc::clone(&self.context),
            expression,
            Rc::clone(&self.ty_context),
        );
        expression_lowerer.lower()
    }
}
