use std::rc::Rc;

use crate::expressions::Expression;

#[derive(Clone)]
pub struct ReturnStatement {
    expression: Rc<Expression>,
}

impl ReturnStatement {
    pub fn new(expression: Rc<Expression>) -> Self {
        Self { expression }
    }

    pub fn get_expression(&self) -> Rc<Expression> {
        Rc::clone(&self.expression)
    }
}
