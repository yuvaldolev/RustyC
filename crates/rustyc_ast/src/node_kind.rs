use crate::NumberNode;

#[derive(Debug)]
pub enum NodeKind {
    Equality,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Negation,
    Number(NumberNode),
}
