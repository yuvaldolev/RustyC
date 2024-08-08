use std::rc::Rc;

use crate::expressions::Expression;

#[derive(Clone)]
pub struct ExpressionStatement {
    expression: Rc<Expression>,
}

impl ExpressionStatement {
    pub fn new(expression: Rc<Expression>) -> Self {
        Self { expression }
    }

    pub fn get_expression(&self) -> Rc<Expression> {
        Rc::clone(&self.expression)
    }
}
