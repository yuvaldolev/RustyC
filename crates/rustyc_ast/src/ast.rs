use std::rc::Rc;

use crate::items::Item;

pub struct Ast {
    items: Vec<Rc<Item>>,
}

impl Ast {
    pub fn new(items: Vec<Rc<Item>>) -> Self {
        Self { items }
    }

    pub fn get_items(&self) -> &[Rc<Item>] {
        &self.items
    }
}
