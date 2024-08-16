use super::{CompoundStatement, ExpressionStatement, IfStatement, LoopStatement, ReturnStatement};

#[derive(Clone)]
pub enum StatementKind {
    Return(ReturnStatement),
    If(IfStatement),
    Loop(LoopStatement),
    Compound(CompoundStatement),
    Expression(ExpressionStatement),
}
