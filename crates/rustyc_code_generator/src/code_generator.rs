use std::rc::Rc;

use rustyc_hir::Hir;

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter, item_generator::ItemGenerator,
};

pub struct CodeGenerator {
    hir: Rc<Hir>,
    instruction_emitter: Aarch64InstructionEmitter,
}

impl CodeGenerator {
    pub fn new(hir: Rc<Hir>) -> Self {
        Self {
            hir,
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        self.instruction_emitter.emit_text_section_directive();

        for item in self.hir.get_items().iter() {
            let item_generator = ItemGenerator::new(Rc::clone(item));
            item_generator.generate()?;
        }

        Ok(())
    }
}
