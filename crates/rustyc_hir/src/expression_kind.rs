use std::rc::Rc;

use crate::{BinaryOperator, Expression, UnaryOperator};

pub enum ExpressionKind {
    Assignment(Rc<Expression>, Rc<Expression>),
    Binary(BinaryOperator, Rc<Expression>, Rc<Expression>),
    Unary(UnaryOperator, Rc<Expression>),
    Variable(String),
    Number(u64),
    FunctionCall(String, Vec<Rc<Expression>>),
}
