use rustyc_ast::Item;

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter, item_generator::ItemGenerator,
};

pub struct CodeGenerator {
    ast: Item,
    instruction_emitter: Aarch64InstructionEmitter,
}

impl CodeGenerator {
    pub fn new(ast: Item) -> Self {
        Self {
            ast,
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        self.instruction_emitter.emit_text_section_directive();

        let item_generator = ItemGenerator::new(&self.ast);
        item_generator.generate()?;

        Ok(())
    }
}
