use crate::Expression;

pub enum StatementKind {
    Expression(Box<Expression>),
    Return(Box<Expression>),
}
