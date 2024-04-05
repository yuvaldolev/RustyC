use std::{cell::RefCell, rc::Rc};

use rustyc_ast::{FunctionItem, Item, ItemKind};

use crate::{
    function::Function, function_generator::FunctionGenerator, label_allocator::LabelAllocator,
};

pub struct ItemGenerator {
    item: Rc<Item>,
    label_allocator: Rc<RefCell<LabelAllocator>>,
}

impl ItemGenerator {
    pub fn new(item: Rc<Item>, label_allocator: Rc<RefCell<LabelAllocator>>) -> Self {
        Self {
            item,
            label_allocator,
        }
    }

    pub fn generate(mut self) -> rustyc_diagnostics::Result<()> {
        match self.item.get_kind() {
            ItemKind::Function(function) => self.generate_function(Rc::clone(function)),
        }
    }

    fn generate_function(&mut self, item: Rc<FunctionItem>) -> rustyc_diagnostics::Result<()> {
        // TODO: In the future the function name will be parsed into the `FunctionItem`
        // struct and thus should be removed from the `Function` struct.
        let generator = FunctionGenerator::new(
            Function::new(String::from("_main"), item),
            Rc::clone(&self.label_allocator),
        );
        generator.generate()
    }
}
