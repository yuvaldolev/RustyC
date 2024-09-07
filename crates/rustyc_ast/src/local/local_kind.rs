use std::rc::Rc;

use crate::expressions::Expression;

#[derive(Clone)]
pub enum LocalKind {
    Declaration,
    Initialization(Rc<Expression>),
}
