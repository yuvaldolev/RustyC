use std::rc::Rc;

use crate::expressions::Expression;

use super::Statement;

#[derive(Clone)]
pub struct IfStatement {
    condition_expression: Rc<Expression>,
    then_statement: Rc<Statement>,
    else_statement: Option<Rc<Statement>>,
}

impl IfStatement {
    pub fn new(
        condition_expression: Rc<Expression>,
        then_statement: Rc<Statement>,
        else_statement: Option<Rc<Statement>>,
    ) -> Self {
        Self {
            condition_expression,
            then_statement,
            else_statement,
        }
    }

    pub fn get_condition_expression(&self) -> Rc<Expression> {
        Rc::clone(&self.condition_expression)
    }

    pub fn get_then_statement(&self) -> Rc<Statement> {
        Rc::clone(&self.then_statement)
    }

    pub fn get_else_statement(&self) -> Option<Rc<Statement>> {
        self.else_statement.clone()
    }
}
