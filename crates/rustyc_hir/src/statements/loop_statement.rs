use std::rc::Rc;

use crate::expressions::Expression;

use super::Statement;

#[derive(Clone, Debug)]
pub struct LoopStatement {
    initialization_statement: Option<Rc<Statement>>,
    condition_expression: Option<Rc<Expression>>,
    incrementation_expression: Option<Rc<Expression>>,
    then_statement: Rc<Statement>,
}

impl LoopStatement {
    pub fn new(
        initialization_statement: Option<Rc<Statement>>,
        condition_expression: Option<Rc<Expression>>,
        incrementation_expression: Option<Rc<Expression>>,
        then_statement: Rc<Statement>,
    ) -> Self {
        Self {
            initialization_statement,
            condition_expression,
            incrementation_expression,
            then_statement,
        }
    }

    pub fn get_initialization_statement(&self) -> Option<Rc<Statement>> {
        self.initialization_statement.clone()
    }

    pub fn get_condition_expression(&self) -> Option<Rc<Expression>> {
        self.condition_expression.clone()
    }

    pub fn get_incrementation_expression(&self) -> Option<Rc<Expression>> {
        self.incrementation_expression.clone()
    }

    pub fn get_then_statement(&self) -> Rc<Statement> {
        Rc::clone(&self.then_statement)
    }
}
