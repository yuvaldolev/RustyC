use std::rc::Rc;

use crate::items::Item;

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
