use std::rc::Rc;

use rustyc_hir::{FunctionItem, Item, ItemKind};

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter, function_generator::FunctionGenerator,
};

pub struct ItemGenerator {
    item: Rc<Item>,
    instruction_emitter: Aarch64InstructionEmitter,
}

impl ItemGenerator {
    pub fn new(item: Rc<Item>) -> Self {
        Self {
            item,
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        self.instruction_emitter.emit_item_separator();

        match self.item.get_kind() {
            ItemKind::Function(function) => self.generate_function(Rc::clone(function)),
        }
    }

    fn generate_function(&self, item: Rc<FunctionItem>) -> rustyc_diagnostics::Result<()> {
        // TODO: In the future the function name will be parsed into the `FunctionItem`
        // struct and thus should be removed from the `Function` struct.
        let generator = FunctionGenerator::new(item);
        generator.generate()
    }
}
