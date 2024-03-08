use rustyc_ast::{FunctionItem, Item, ItemKind};

use crate::{function::Function, function_generator::FunctionGenerator};

pub struct ItemGenerator<'ast> {
    item: &'ast Item,
}

impl<'ast> ItemGenerator<'ast> {
    pub fn new(item: &'ast Item) -> Self {
        Self { item }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        match self.item.get_kind() {
            ItemKind::Function(function) => Self::generate_function(function),
        }
    }

    fn generate_function(item: &FunctionItem) -> rustyc_diagnostics::Result<()> {
        // TODO: In the future the function name will be parsed into the `FunctionItem`
        // struct and thus should be removed from the `Function` struct.
        let generator = FunctionGenerator::new(Function::new(String::from("_main"), item));
        generator.generate()
    }
}
