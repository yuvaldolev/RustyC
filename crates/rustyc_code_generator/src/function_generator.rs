use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter, block_generator::BlockGenerator,
    function::Function,
};

pub struct FunctionGenerator<'ast> {
    function: Function<'ast>,
    instruction_emitter: Aarch64InstructionEmitter,
}

impl<'ast> FunctionGenerator<'ast> {
    pub fn new(function: Function<'ast>) -> Self {
        Self {
            function,
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        self.generate_prologue();

        let block_generator = BlockGenerator::new(
            self.function.get_item().get_body(),
            self.function.get_local_variables(),
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
