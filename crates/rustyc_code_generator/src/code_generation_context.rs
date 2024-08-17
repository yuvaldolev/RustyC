use std::collections::HashMap;

use rustyc_hir::items::FunctionItem;

use crate::{label_allocator::LabelAllocator, variable_properties::VariableProperties};

pub struct CodeGenerationContext {
    label_allocator: LabelAllocator,
    stack_size: i64,
    local_variables: HashMap<String, VariableProperties>,
}

impl CodeGenerationContext {
    pub fn new() -> Self {
        Self {
            label_allocator: LabelAllocator::new(),
            stack_size: 0,
            local_variables: HashMap::new(),
        }
    }

    pub fn enter_function(&mut self, function: &FunctionItem) {
        self.label_allocator.set_prefix(function.get_name());

        let mut offset: i64 = 0;

        for variable in function.get_local_variables().iter() {
            offset += 8;
            self.local_variables
                .entry(variable.clone())
                .or_insert(VariableProperties::new(-offset));
        }

        self.stack_size = Self::align_to(offset, 16);
    }

    pub fn exit_function(&mut self) {
        self.local_variables.clear();
    }

    pub fn get_label_allocator(&mut self) -> &mut LabelAllocator {
        &mut self.label_allocator
    }

    pub fn get_stack_size(&self) -> i64 {
        self.stack_size
    }

    pub fn get_local_variables(&self) -> &HashMap<String, VariableProperties> {
        &self.local_variables
    }

    fn align_to(value: i64, alignment: i64) -> i64 {
        (value + alignment - 1) / alignment * alignment
    }
}
