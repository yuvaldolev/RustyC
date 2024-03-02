use rustyc_ast::{FunctionItem, Item, ItemKind};

use crate::function_generator::FunctionGenerator;

pub struct CodeGenerator {
    ast: Item,
}

impl CodeGenerator {
    pub fn new(ast: Item) -> Self {
        Self { ast }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        // TODO: Move to instruction generator.
        println!(".text");
        println!();

        // TODO: If all the clones of different items turn to be a performance
        // bottleneck, we probably can change all this to move each item to its
        // relevant generator instead of cloninig all items.
        Self::generate_item(&self.ast)?;

        Ok(())
    }

    fn generate_item(item: &Item) -> rustyc_diagnostics::Result<()> {
        match item.get_kind() {
            ItemKind::Function(function) => Self::generate_function(function)?,
        }

        Ok(())
    }

    fn generate_function(function: &FunctionItem) -> rustyc_diagnostics::Result<()> {
        let generator = FunctionGenerator::new(String::from("_main"), function.clone());
        generator.generate()?;

        Ok(())
    }
}
