use std::rc::Rc;

pub struct ExpressionLowerer {
    expression: Rc<rustyc_ast::Expression>,
    ty_context: Rc<rustyc_ty::TyContext>,
}

impl ExpressionLowerer {
    pub fn new(
        expression: Rc<rustyc_ast::Expression>,
        ty_context: Rc<rustyc_ty::TyContext>,
    ) -> Self {
        Self {
            expression,
            ty_context,
        }
    }

    pub fn lower(self) -> Rc<rustyc_hir::Expression> {
        let hir_expression_kind = match self.expression.get_kind() {
            rustyc_ast::ExpressionKind::Assignment(left, right) => {
                rustyc_hir::ExpressionKind::Assignment(
                    self.lower_expression(Rc::clone(left)),
                    self.lower_expression(Rc::clone(right)),
                )
            }
            rustyc_ast::ExpressionKind::Binary(operator, left, right) => {
                rustyc_hir::ExpressionKind::Binary(
                    Self::lower_binary_operator(operator),
                    self.lower_expression(Rc::clone(left)),
                    self.lower_expression(Rc::clone(right)),
                )
            }
            rustyc_ast::ExpressionKind::Unary(operator, right) => {
                rustyc_hir::ExpressionKind::Unary(
                    Self::lower_unary_operator(operator),
                    self.lower_expression(Rc::clone(right)),
                )
            }
            rustyc_ast::ExpressionKind::Variable(variable) => {
                rustyc_hir::ExpressionKind::Variable(variable.clone())
            }
            rustyc_ast::ExpressionKind::Number(number) => {
                rustyc_hir::ExpressionKind::Number(*number)
            }
            rustyc_ast::ExpressionKind::FunctionCall(name, arguments) => {
                rustyc_hir::ExpressionKind::FunctionCall(
                    name.clone(),
                    arguments
                        .iter()
                        .map(|argument| self.lower_expression(Rc::clone(argument)))
                        .collect(),
                )
            }
        };

        Rc::new(rustyc_hir::Expression::new(
            hir_expression_kind,
            self.ty_context.get_int(), // TODO: Modify
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

    fn lower_expression(
        &self,
        expression: Rc<rustyc_ast::Expression>,
    ) -> Rc<rustyc_hir::Expression> {
        let expression_lowerer = Self::new(expression, Rc::clone(&self.ty_context));
        expression_lowerer.lower()
    }
}
