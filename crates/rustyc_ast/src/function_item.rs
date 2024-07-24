use std::rc::Rc;

use crate::block::Block;

pub struct FunctionItem {
    name: String,
    body: Rc<Block>,
    local_variables: Vec<String>,
}

impl FunctionItem {
    pub fn new(name: String, body: Rc<Block>, local_variables: Vec<String>) -> Self {
        Self {
            name,
            body,
            local_variables,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_body(&self) -> Rc<Block> {
        Rc::clone(&self.body)
    }

    pub fn get_local_variables(&self) -> &[String] {
        &self.local_variables
    }
}
