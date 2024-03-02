use crate::{Block, Expression};

#[derive(Clone)]
pub enum StatementKind {
    Compound(Block),
    Return(Box<Expression>),
    Expression(Box<Expression>),
}
