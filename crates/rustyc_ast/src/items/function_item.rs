use std::rc::Rc;

use crate::Block;

pub struct FunctionItem {
    name: String,
    parameters: Vec<String>,
    body: Rc<Block>,
}

impl FunctionItem {
    pub fn new(name: String, parameters: Vec<String>, body: Rc<Block>) -> Self {
        Self {
            name,
            parameters,
            body,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_parameters(&self) -> &[String] {
        &self.parameters
    }

    pub fn get_body(&self) -> Rc<Block> {
        Rc::clone(&self.body)
    }
}
