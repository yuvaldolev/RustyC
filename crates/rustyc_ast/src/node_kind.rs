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
    Number(u64),
    Variable(char),
}
