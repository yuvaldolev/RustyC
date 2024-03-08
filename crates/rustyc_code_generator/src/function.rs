use std::collections::HashMap;

use rustyc_ast::FunctionItem;

use crate::variable_properties::VariableProperties;

pub struct Function<'ast> {
    name: String, // TODO: Remove when parsed into `FunctionItem`.
    item: &'ast FunctionItem,
    stack_size: i64,
    local_variables: HashMap<String, VariableProperties>,
}

impl<'ast> Function<'ast> {
    pub fn new(name: String, item: &'ast FunctionItem) -> Self {
        let mut local_variables: HashMap<String, VariableProperties> = HashMap::new();
        let mut offset: i64 = 0;

        for variable in item.get_local_variables().iter() {
            offset += 8;
            local_variables
                .entry(variable.clone())
                .or_insert(VariableProperties::new(-offset));
        }

        Self {
            name,
            item,
            stack_size: Self::align_to(offset, 16),
            local_variables,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_item(&self) -> &FunctionItem {
        self.item
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
