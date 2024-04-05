use std::rc::Rc;

use crate::{Block, Expression, Statement};

#[derive(Clone)]
pub enum StatementKind {
    Return(Rc<Expression>),
    If(Rc<Expression>, Rc<Statement>, Option<Rc<Statement>>),
    Compound(Rc<Block>),
    Expression(Rc<Expression>),
}
