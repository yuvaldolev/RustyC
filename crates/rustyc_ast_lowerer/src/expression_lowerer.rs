use std::{cell::RefCell, rc::Rc};

use rustyc_ty::{Ty, TyId};

pub struct ExpressionLowerer {
    expression: Rc<rustyc_ast::Expression>,
    ty_context: Rc<RefCell<rustyc_ty::TyContext>>,
}

impl ExpressionLowerer {
    pub fn new(
        expression: Rc<rustyc_ast::Expression>,
        ty_context: Rc<RefCell<rustyc_ty::TyContext>>,
    ) -> Self {
        Self {
            expression,
            ty_context,
        }
    }

    pub fn lower(self) -> Rc<rustyc_hir::Expression> {
        let (hir_expression_kind, ty) = match self.expression.get_kind() {
            rustyc_ast::ExpressionKind::Assignment(left, right) => {
                self.lower_assignment(Rc::clone(left), Rc::clone(right))
            }
            rustyc_ast::ExpressionKind::Binary(operator, left, right) => {
                self.lower_binary(operator, Rc::clone(left), Rc::clone(right))
            }
            rustyc_ast::ExpressionKind::Unary(operator, right) => {
                self.lower_unary(operator, Rc::clone(right))
            }
            rustyc_ast::ExpressionKind::Variable(variable) => self.lower_variable(variable),
            rustyc_ast::ExpressionKind::Number(number) => self.lower_number(*number),
            rustyc_ast::ExpressionKind::FunctionCall(name, arguments) => {
                self.lower_function_call(name, arguments)
            }
        };

        Rc::new(rustyc_hir::Expression::new(
            hir_expression_kind,
            ty,
            self.expression.get_span().clone(),
        ))
    }

    fn lower_binary_operator(operator: &rustyc_ast::BinaryOperator) -> rustyc_hir::BinaryOperator {
        match operator {
            rustyc_ast::BinaryOperator::Equal => rustyc_hir::BinaryOperator::Equal,
            rustyc_ast::BinaryOperator::NotEqual => rustyc_hir::BinaryOperator::NotEqual,
            rustyc_ast::BinaryOperator::LessThan => rustyc_hir::BinaryOperator::LessThan,
            rustyc_ast::BinaryOperator::LessThanOrEqual => {
                rustyc_hir::BinaryOperator::LessThanOrEqual
            }
            rustyc_ast::BinaryOperator::Add => rustyc_hir::BinaryOperator::Add,
            rustyc_ast::BinaryOperator::Subtract => rustyc_hir::BinaryOperator::Subtract,
            rustyc_ast::BinaryOperator::Multiply => rustyc_hir::BinaryOperator::Multiply,
            rustyc_ast::BinaryOperator::Divide => rustyc_hir::BinaryOperator::Divide,
        }
    }

    fn lower_unary_operator(operator: &rustyc_ast::UnaryOperator) -> rustyc_hir::UnaryOperator {
        match operator {
            rustyc_ast::UnaryOperator::Negate => rustyc_hir::UnaryOperator::Negate,
            rustyc_ast::UnaryOperator::AddressOf => rustyc_hir::UnaryOperator::AddressOf,
            rustyc_ast::UnaryOperator::Dereference => rustyc_hir::UnaryOperator::Dereference,
        }
    }

    fn lower_assignment(
        &self,
        left: Rc<rustyc_ast::Expression>,
        right: Rc<rustyc_ast::Expression>,
    ) -> (rustyc_hir::ExpressionKind, TyId) {
        let hir_left = self.lower_expression(left);
        let hir_right = self.lower_expression(right);

        let ty = hir_left.get_ty();

        (
            rustyc_hir::ExpressionKind::Assignment(hir_left, hir_right),
            ty,
        )
    }

    fn lower_binary(
        &self,
        operator: &rustyc_ast::BinaryOperator,
        left: Rc<rustyc_ast::Expression>,
        right: Rc<rustyc_ast::Expression>,
    ) -> (rustyc_hir::ExpressionKind, TyId) {
        let hir_left = self.lower_expression(left);
        let hir_right = self.lower_expression(right);

        let ty = hir_left.get_ty();

        match operator {
            rustyc_ast::BinaryOperator::Add => self.lower_add(hir_left, hir_right),
            rustyc_ast::BinaryOperator::Subtract => self.lower_subtract(hir_left, hir_right),
            _ => (
                rustyc_hir::ExpressionKind::Binary(
                    Self::lower_binary_operator(operator),
                    hir_left,
                    hir_right,
                ),
                ty,
            ),
        }
    }

    fn lower_unary(
        &self,
        operator: &rustyc_ast::UnaryOperator,
        right: Rc<rustyc_ast::Expression>,
    ) -> (rustyc_hir::ExpressionKind, TyId) {
        let hir_right = self.lower_expression(right);

        let ty = match operator {
            rustyc_ast::UnaryOperator::Negate => hir_right.get_ty(),
            rustyc_ast::UnaryOperator::AddressOf => self
                .ty_context
                .borrow_mut()
                .register(Ty::Pointer(hir_right.get_ty())),
            rustyc_ast::UnaryOperator::Dereference => {
                if let Ty::Pointer(base) = self.ty_context.borrow().get(hir_right.get_ty()) {
                    *base
                } else {
                    self.ty_context.borrow_mut().register(Ty::Int)
                }
            }
        };

        (
            rustyc_hir::ExpressionKind::Unary(Self::lower_unary_operator(operator), hir_right),
            ty,
        )
    }

    fn lower_variable(&self, variable: &str) -> (rustyc_hir::ExpressionKind, TyId) {
        (
            rustyc_hir::ExpressionKind::Variable(variable.to_owned()),
            self.ty_context.borrow_mut().register(Ty::Int),
        )
    }

    fn lower_number(&self, number: u64) -> (rustyc_hir::ExpressionKind, TyId) {
        (
            rustyc_hir::ExpressionKind::Number(number),
            self.ty_context.borrow_mut().register(Ty::Int),
        )
    }

    fn lower_function_call(
        &self,
        name: &str,
        arguments: &[Rc<rustyc_ast::Expression>],
    ) -> (rustyc_hir::ExpressionKind, TyId) {
        (
            rustyc_hir::ExpressionKind::FunctionCall(
                name.to_owned(),
                arguments
                    .iter()
                    .map(|argument| self.lower_expression(Rc::clone(argument)))
                    .collect(),
            ),
            self.ty_context.borrow_mut().register(Ty::Int),
        )
    }

    fn lower_add(
        &self,
        left: Rc<rustyc_hir::Expression>,
        right: Rc<rustyc_hir::Expression>,
    ) -> (rustyc_hir::ExpressionKind, TyId) {
        if let Ty::Pointer(_) = self.ty_context.borrow().get(left.get_ty()) {
            if let Ty::Int = self.ty_context.borrow().get(right.get_ty()) {
                return self.lower_pointer_number_arithmetic(
                    left,
                    right,
                    rustyc_hir::BinaryOperator::Add,
                );
            }
        }

        if let Ty::Int = self.ty_context.borrow().get(left.get_ty()) {
            if let Ty::Pointer(_) = self.ty_context.borrow().get(right.get_ty()) {
                return self.lower_pointer_number_arithmetic(
                    right,
                    left,
                    rustyc_hir::BinaryOperator::Add,
                );
            }
        }

        let ty = left.get_ty();

        (
            rustyc_hir::ExpressionKind::Binary(rustyc_hir::BinaryOperator::Add, left, right),
            ty,
        )
    }

    fn lower_subtract(
        &self,
        left: Rc<rustyc_hir::Expression>,
        right: Rc<rustyc_hir::Expression>,
    ) -> (rustyc_hir::ExpressionKind, TyId) {
        if let Ty::Pointer(_) = self.ty_context.borrow().get(left.get_ty()) {
            if let Ty::Int = self.ty_context.borrow().get(right.get_ty()) {
                return self.lower_pointer_number_arithmetic(
                    left,
                    right,
                    rustyc_hir::BinaryOperator::Subtract,
                );
            }
        }

        if let Ty::Pointer(_) = self.ty_context.borrow().get(left.get_ty()) {
            if let Ty::Pointer(_) = self.ty_context.borrow().get(right.get_ty()) {
                return self.lower_pointer_pointer_subtract(left, right);
            }
        }

        let ty = left.get_ty();

        (
            rustyc_hir::ExpressionKind::Binary(rustyc_hir::BinaryOperator::Subtract, left, right),
            ty,
        )
    }

    fn lower_pointer_number_arithmetic(
        &self,
        pointer: Rc<rustyc_hir::Expression>,
        number: Rc<rustyc_hir::Expression>,
        operator: rustyc_hir::BinaryOperator,
    ) -> (rustyc_hir::ExpressionKind, TyId) {
        let pointer_ty = pointer.get_ty();
        let number_ty = number.get_ty();

        let number_span = number.get_span().clone();

        (
            rustyc_hir::ExpressionKind::Binary(
                operator,
                pointer,
                Rc::new(rustyc_hir::Expression::new(
                    rustyc_hir::ExpressionKind::Binary(
                        rustyc_hir::BinaryOperator::Multiply,
                        number,
                        Rc::new(rustyc_hir::Expression::new(
                            rustyc_hir::ExpressionKind::Number(8),
                            self.ty_context.borrow_mut().register(Ty::Int),
                            number_span.clone(),
                        )),
                    ),
                    number_ty,
                    number_span,
                )),
            ),
            pointer_ty,
        )
    }

    fn lower_pointer_pointer_subtract(
        &self,
        left: Rc<rustyc_hir::Expression>,
        right: Rc<rustyc_hir::Expression>,
    ) -> (rustyc_hir::ExpressionKind, TyId) {
        (
            rustyc_hir::ExpressionKind::Binary(
                rustyc_hir::BinaryOperator::Divide,
                Rc::new(rustyc_hir::Expression::new(
                    rustyc_hir::ExpressionKind::Binary(
                        rustyc_hir::BinaryOperator::Subtract,
                        left,
                        right,
                    ),
                    self.ty_context.borrow_mut().register(Ty::Int),
                    self.expression.get_span().clone(),
                )),
                Rc::new(rustyc_hir::Expression::new(
                    rustyc_hir::ExpressionKind::Number(8),
                    self.ty_context.borrow_mut().register(Ty::Int),
                    self.expression.get_span().clone(),
                )),
            ),
            self.ty_context.borrow_mut().register(Ty::Int),
        )
    }

    fn lower_expression(
        &self,
        expression: Rc<rustyc_ast::Expression>,
    ) -> Rc<rustyc_hir::Expression> {
        let expression_lowerer = Self::new(expression, Rc::clone(&self.ty_context));
        expression_lowerer.lower()
    }
}
