use std::rc::Rc;

use crate::items::Item;

// TODO: There are multiple crates which are basically implemented as follows:
// 1. Traversing the HIR
// 2. Performing some kind of action on the different nodes
//
// Should we create a generic trait for this repeated pattern?

pub struct Hir {
    items: Vec<Rc<Item>>,
}

impl Hir {
    pub fn new(items: Vec<Rc<Item>>) -> Self {
        Self { items }
    }

    pub fn get_items(&self) -> &[Rc<Item>] {
        &self.items
    }
}
