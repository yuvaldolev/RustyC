use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rustyc_ast::Block;

use crate::{
    label_allocator::LabelAllocator, statement_generator::StatementGenerator,
    variable_properties::VariableProperties,
};

pub struct BlockGenerator {
    block: Rc<Block>,
    local_variables: Rc<HashMap<String, VariableProperties>>,
    label_allocator: Rc<RefCell<LabelAllocator>>,
}

impl BlockGenerator {
    pub fn new(
        block: Rc<Block>,
        local_variables: Rc<HashMap<String, VariableProperties>>,
        label_allocator: Rc<RefCell<LabelAllocator>>,
    ) -> Self {
        Self {
            block,
            local_variables,
            label_allocator,
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        for statement in self.block.get_statements().iter() {
            let statement_generator = StatementGenerator::new(
                Rc::clone(statement),
                Rc::clone(&self.local_variables),
                Rc::clone(&self.label_allocator),
            );
            statement_generator.generate()?;
        }

        Ok(())
    }
}
