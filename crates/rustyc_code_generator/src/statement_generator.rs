use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter,
    code_generation_context::CodeGenerationContext,
};

pub struct StatementGenerator {
    instruction_emitter: Aarch64InstructionEmitter,
}

impl StatementGenerator {
    pub fn new() -> Self {
        Self {
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate_return_statement_post(&self, context: &mut CodeGenerationContext) {
        self.instruction_emitter.emit_branch(
            context
                .get_label_allocator()
                .allocate_global("return")
                .as_str(),
        );
    }

    pub fn generate_if_statement_condition_post(&self, else_label: &str) {
        self.instruction_emitter.emit_comparison("x0", "#0");
        self.instruction_emitter.emit_branch_equals(&else_label);
    }

    pub fn generate_if_statement_then_post(&self, end_label: &str) {
        self.instruction_emitter.emit_branch(&end_label);
    }

    pub fn generate_if_statement_else_pre(&self, else_label: &str) {
        self.instruction_emitter.emit_label(&else_label);
    }

    pub fn generate_if_statement_else_post(&self, end_label: &str) {
        self.instruction_emitter.emit_label(&end_label);
    }

    pub fn generate_loop_statement_initialization_post(&self, begin_label: &str) {
        self.instruction_emitter.emit_label(&begin_label);
    }

    pub fn generate_loop_statement_condition_post(&self, end_label: &str) {
        self.instruction_emitter.emit_comparison("x0", "#0");
        self.instruction_emitter.emit_branch_equals(&end_label);
    }

    pub fn generate_loop_statement_incrementation_post(&self, begin_label: &str, end_label: &str) {
        self.instruction_emitter.emit_branch(&begin_label);
        self.instruction_emitter.emit_label(&end_label);
    }
}
