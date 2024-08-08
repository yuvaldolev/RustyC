use std::rc::Rc;

use super::Expression;

#[derive(Clone, Debug)]
pub struct FunctionCallExpression {
    name: String,
    arguments: Vec<Rc<Expression>>,
}

impl FunctionCallExpression {
    pub fn new(name: String, arguments: Vec<Rc<Expression>>) -> Self {
        Self { name, arguments }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_arguments(&self) -> &[Rc<Expression>] {
        &self.arguments
    }
}
