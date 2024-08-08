use std::rc::Rc;

use super::Expression;

#[derive(Clone, Debug)]
pub struct AssignmentExpression {
    left: Rc<Expression>,
    right: Rc<Expression>,
}

impl AssignmentExpression {
    pub fn new(left: Rc<Expression>, right: Rc<Expression>) -> Self {
        Self { left, right }
    }

    pub fn get_left(&self) -> Rc<Expression> {
        Rc::clone(&self.left)
    }

    pub fn get_right(&self) -> Rc<Expression> {
        Rc::clone(&self.right)
    }
}
