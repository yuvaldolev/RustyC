use std::rc::Rc;

use super::{BinaryOperator, Expression};

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    operator: BinaryOperator,
    left: Rc<Expression>,
    right: Rc<Expression>,
}

impl BinaryExpression {
    pub fn new(operator: BinaryOperator, left: Rc<Expression>, right: Rc<Expression>) -> Self {
        Self {
            operator,
            left,
            right,
        }
    }

    pub fn get_operator(&self) -> &BinaryOperator {
        &self.operator
    }

    pub fn get_left(&self) -> Rc<Expression> {
        Rc::clone(&self.left)
    }

    pub fn get_right(&self) -> Rc<Expression> {
        Rc::clone(&self.right)
    }
}
