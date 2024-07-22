use std::rc::Rc;

use crate::{binary_operator::BinaryOperator, unary_operator::UnaryOperator, Expression};

#[derive(Clone, Debug)]
pub enum ExpressionKind {
    Assignment(Rc<Expression>, Rc<Expression>),
    Binary(BinaryOperator, Rc<Expression>, Rc<Expression>),
    Unary(UnaryOperator, Rc<Expression>),
    Variable(String),
    Number(u64),
    FunctionCall(String),
}
