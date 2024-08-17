use crate::aarch64_instruction_emitter::Aarch64InstructionEmitter;

pub struct ItemGenerator {
    instruction_emitter: Aarch64InstructionEmitter,
}

impl ItemGenerator {
    pub fn new() -> Self {
        Self {
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate_item_separator(&self) {
        self.instruction_emitter.emit_item_separator();
    }
}
