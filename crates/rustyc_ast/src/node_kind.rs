use crate::{NumberNode, VariableNode};

#[derive(Debug)]
pub enum NodeKind {
    Assignment,
    Equality,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Negation,
    ExpressionStatement,
    Number(NumberNode),
    Variable(VariableNode),
}
