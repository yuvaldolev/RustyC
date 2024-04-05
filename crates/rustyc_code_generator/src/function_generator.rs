use std::{cell::RefCell, rc::Rc};

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter, block_generator::BlockGenerator,
    function::Function, label_allocator::LabelAllocator,
};

pub struct FunctionGenerator {
    function: Function,
    label_allocator: Rc<RefCell<LabelAllocator>>,
    instruction_emitter: Aarch64InstructionEmitter,
}

impl FunctionGenerator {
    pub fn new(function: Function, label_allocator: Rc<RefCell<LabelAllocator>>) -> Self {
        Self {
            function,
            label_allocator,
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        self.generate_prologue();

        let block_generator = BlockGenerator::new(
            self.function.get_item().get_body(),
            self.function.get_local_variables(),
            Rc::clone(&self.label_allocator),
        );
        block_generator.generate()?;

        self.generate_epilogue();

        Ok(())
    }

    fn generate_prologue(&self) {
        self.instruction_emitter
            .emit_global(self.function.get_name());
        self.instruction_emitter
            .emit_label(self.function.get_name());

        self.instruction_emitter.emit_push("fp");
        self.instruction_emitter.emit_move("sp", "fp");
        self.instruction_emitter.emit_subtract(
            "sp",
            self.function.get_stack_size().to_string().as_str(),
            "sp",
        );
    }

    fn generate_epilogue(&self) {
        self.instruction_emitter.emit_label(".L.return");

        self.instruction_emitter.emit_move("fp", "sp");
        self.instruction_emitter.emit_pop("fp");
        self.instruction_emitter.emit_return();
    }
}
