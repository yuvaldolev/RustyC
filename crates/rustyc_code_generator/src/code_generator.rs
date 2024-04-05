use std::{cell::RefCell, rc::Rc};

use rustyc_ast::Item;

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter, item_generator::ItemGenerator,
    label_allocator::LabelAllocator,
};

pub struct CodeGenerator {
    ast: Rc<Item>,
    instruction_emitter: Aarch64InstructionEmitter,
    label_allocator: Rc<RefCell<LabelAllocator>>,
}

impl CodeGenerator {
    pub fn new(ast: Item) -> Self {
        Self {
            ast: Rc::new(ast),
            instruction_emitter: Aarch64InstructionEmitter::new(),
            label_allocator: Rc::new(RefCell::new(LabelAllocator::new())),
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        self.instruction_emitter.emit_text_section_directive();

        let item_generator =
            ItemGenerator::new(Rc::clone(&self.ast), Rc::clone(&self.label_allocator));
        item_generator.generate()?;

        Ok(())
    }
}
