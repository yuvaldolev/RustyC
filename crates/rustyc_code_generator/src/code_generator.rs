use std::rc::Rc;

use rustyc_hir::Item;

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter, item_generator::ItemGenerator,
};

pub struct CodeGenerator {
    ast: Vec<Rc<Item>>,
    instruction_emitter: Aarch64InstructionEmitter,
}

impl CodeGenerator {
    pub fn new(ast: Vec<Rc<Item>>) -> Self {
        Self {
            ast,
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        self.instruction_emitter.emit_text_section_directive();

        for item in self.ast.iter() {
            let item_generator = ItemGenerator::new(Rc::clone(item));
            item_generator.generate()?;
        }

        Ok(())
    }
}
