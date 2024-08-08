use std::rc::Rc;

use crate::{Block, Expression, Statement};

// TODO: Convert all statement kinds to dedicated structs (relevant for
// expressions and as well, and for the HIR).

#[derive(Clone)]
pub enum StatementKind {
    Return(Rc<Expression>),
    If(Rc<Expression>, Rc<Statement>, Option<Rc<Statement>>),
    Loop(
        Option<Rc<Statement>>,
        Option<Rc<Expression>>,
        Option<Rc<Expression>>,
        Rc<Statement>,
    ),
    Compound(Rc<Block>),
    Expression(Rc<Expression>),
}
