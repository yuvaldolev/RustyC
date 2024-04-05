use std::rc::Rc;

use crate::block::Block;

pub struct FunctionItem {
    body: Rc<Block>,
    local_variables: Vec<String>,
}

impl FunctionItem {
    pub fn new(body: Rc<Block>, local_variables: Vec<String>) -> Self {
        Self {
            body,
            local_variables,
        }
    }

    pub fn get_body(&self) -> Rc<Block> {
        Rc::clone(&self.body)
    }

    pub fn get_local_variables(&self) -> &[String] {
        &self.local_variables
    }
}
