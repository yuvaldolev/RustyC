use super::{
    AssignmentExpression, BinaryExpression, FunctionCallExpression, NumberExpression,
    UnaryExpression, VariableReferenceExpression,
};

#[derive(Clone, Debug)]
pub enum ExpressionKind {
    Assignment(AssignmentExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    VariableReference(VariableReferenceExpression),
    Number(NumberExpression),
    FunctionCall(FunctionCallExpression),
}
