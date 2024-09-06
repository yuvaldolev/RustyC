use std::rc::Rc;

use rustyc_index::IndexVec;

use crate::{Block, Local, LocalId};

pub struct FunctionItem {
    name: String,
    locals: IndexVec<LocalId, Local>,
    parameters: Vec<LocalId>,
    body: Rc<Block>,
    local_variables: Vec<LocalId>,
}

impl FunctionItem {
    pub fn new(
        name: String,
        locals: IndexVec<LocalId, Local>,
        parameters: Vec<LocalId>,
        body: Rc<Block>,
        local_variables: Vec<LocalId>,
    ) -> Self {
        Self {
            name,
            locals,
            parameters,
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
}
