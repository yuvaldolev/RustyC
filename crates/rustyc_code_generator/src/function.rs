use std::{collections::HashMap, rc::Rc};

use rustyc_ast::FunctionItem;

use crate::variable_properties::VariableProperties;

pub struct Function {
    item: Rc<FunctionItem>,
    stack_size: i64,
    local_variables: Rc<HashMap<String, VariableProperties>>,
}

impl Function {
    pub fn new(item: Rc<FunctionItem>) -> Self {
        let mut local_variables: HashMap<String, VariableProperties> = HashMap::new();
        let mut offset: i64 = 0;

        for variable in item.get_local_variables().iter() {
            offset += 8;
            local_variables
                .entry(variable.clone())
                .or_insert(VariableProperties::new(-offset));
        }

        Self {
            item,
            stack_size: Self::align_to(offset, 16),
            local_variables: Rc::new(local_variables),
        }
    }

    pub fn get_item(&self) -> &FunctionItem {
        &self.item
    }

    pub fn get_stack_size(&self) -> i64 {
        self.stack_size
    }

    pub fn get_local_variables(&self) -> Rc<HashMap<String, VariableProperties>> {
        Rc::clone(&self.local_variables)
    }

    fn align_to(value: i64, alignment: i64) -> i64 {
        (value + alignment - 1) / alignment * alignment
    }
}
