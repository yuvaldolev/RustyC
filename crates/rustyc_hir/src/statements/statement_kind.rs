use super::{CompoundStatement, ExpressionStatement, IfStatement, LoopStatement, ReturnStatement};

#[derive(Clone, Debug)]
pub enum StatementKind {
    Return(ReturnStatement),
    If(IfStatement),
    Loop(LoopStatement),
    Compound(CompoundStatement),
    Expression(ExpressionStatement),
}
