use crate::{binary_operator::BinaryOperator, unary_operator::UnaryOperator, Expression};

#[derive(Debug)]
pub enum ExpressionKind {
    Assignment(Box<Expression>, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
    Variable(String),
    Number(u64),
}
