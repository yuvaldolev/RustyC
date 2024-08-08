use std::rc::Rc;

use crate::Block;

#[derive(Clone)]
pub struct CompoundStatement {
    block: Rc<Block>,
}

impl CompoundStatement {
    pub fn new(block: Rc<Block>) -> Self {
        Self { block }
    }

    pub fn get_block(&self) -> Rc<Block> {
        Rc::clone(&self.block)
    }
}
