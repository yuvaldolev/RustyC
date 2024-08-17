use rustyc_hir::items::FunctionItem;

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter,
    code_generation_context::CodeGenerationContext,
};

pub struct FunctionGenerator {
    instruction_emitter: Aarch64InstructionEmitter,
}

impl FunctionGenerator {
    pub fn new() -> Self {
        Self {
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate_prologue(&self, function: &FunctionItem, context: &CodeGenerationContext) {
        // TODO: This logic is only relevant to macOS.
        // This would need to be abstracted somehow when adding support
        // for other platforms.
        let function_name = format!("_{}", function.get_name());
        self.instruction_emitter.emit_global(&function_name);
        self.instruction_emitter.emit_label(&function_name);

        self.instruction_emitter.emit_push_pair("fp", "lr");
        self.instruction_emitter.emit_move_registers("sp", "fp");
        // TODO: Does this really work (immediate stack size is missing a hashtag)?
        self.instruction_emitter.emit_subtract(
            "sp",
            context.get_stack_size().to_string().as_str(),
            "sp",
        );
    }

    pub fn generate_push_parameters_to_stack(
        &self,
        function: &FunctionItem,
        context: &CodeGenerationContext,
    ) {
        for (index, parameter) in function.get_parameters().iter().enumerate() {
            // TODO: Emit an error if the variable is not found, instead of panicking.
            self.instruction_emitter.emit_store_offset(
                self.instruction_emitter
                    .get_function_parameter_register(index),
                "fp",
                context
                    .get_local_variables()
                    .get(parameter)
                    .unwrap()
                    .get_offset(),
            );
        }
    }

    pub fn generate_epilogue(&self, context: &mut CodeGenerationContext) {
        self.instruction_emitter.emit_label(
            context
                .get_label_allocator()
                .allocate_global("return")
                .as_str(),
        );

        self.instruction_emitter.emit_move_registers("fp", "sp");
        self.instruction_emitter.emit_pop_pair("fp", "lr");
        self.instruction_emitter.emit_return();
    }
}
