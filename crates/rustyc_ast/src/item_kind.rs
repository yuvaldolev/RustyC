use std::rc::Rc;

use crate::function_item::FunctionItem;

pub enum ItemKind {
    Function(Rc<FunctionItem>),
}
