use crate::Expression;

pub enum StatementKind {
    Expression(Box<Expression>),
}
