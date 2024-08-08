use super::{CompoundStatement, ExpressionStatement, IfStatement, LoopStatement, ReturnStatement};

// TODO: Convert all statement kinds to dedicated structs (relevant for
// expressions and as well, and for the HIR).

#[derive(Clone, Debug)]
pub enum StatementKind {
    Return(ReturnStatement),
    If(IfStatement),
    Loop(LoopStatement),
    Compound(CompoundStatement),
    Expression(ExpressionStatement),
}
