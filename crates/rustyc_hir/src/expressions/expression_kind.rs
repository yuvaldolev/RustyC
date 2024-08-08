use super::{
    AssignmentExpression, BinaryExpression, FunctionCallExpression, NumberExpression,
    UnaryExpression, VariableExpression,
};

#[derive(Clone, Debug)]
pub enum ExpressionKind {
    Assignment(AssignmentExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Variable(VariableExpression),
    Number(NumberExpression),
    FunctionCall(FunctionCallExpression),
}
