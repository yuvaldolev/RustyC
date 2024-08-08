use std::rc::Rc;

use super::{Expression, UnaryOperator};

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    operator: UnaryOperator,
    operand: Rc<Expression>,
}

impl UnaryExpression {
    pub fn new(operator: UnaryOperator, operand: Rc<Expression>) -> Self {
        Self { operator, operand }
    }

    pub fn get_operator(&self) -> &UnaryOperator {
        &self.operator
    }

    pub fn get_operand(&self) -> Rc<Expression> {
        Rc::clone(&self.operand)
    }
}
