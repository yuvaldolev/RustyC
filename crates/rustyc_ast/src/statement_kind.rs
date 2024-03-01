use crate::{Block, Expression};

pub enum StatementKind {
    Compound(Block),
    Return(Box<Expression>),
    Expression(Box<Expression>),
}
