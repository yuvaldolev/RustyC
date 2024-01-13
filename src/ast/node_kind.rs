use super::NumberNode;

#[derive(Debug)]
pub enum NodeKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Number(NumberNode),
}
