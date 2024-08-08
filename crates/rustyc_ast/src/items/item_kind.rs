use std::rc::Rc;

use super::FunctionItem;

pub enum ItemKind {
    Function(Rc<FunctionItem>),
}
