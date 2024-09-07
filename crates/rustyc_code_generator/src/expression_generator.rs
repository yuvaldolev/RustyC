use rustyc_hir::expressions::{
    FunctionCallExpression, NumberExpression, VariableReferenceExpression,
};

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter,
    code_generation_context::CodeGenerationContext,
};

pub struct ExpressionGenerator {
    instruction_emitter: Aarch64InstructionEmitter,
}

impl ExpressionGenerator {
    pub fn new() -> Self {
        Self {
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate_assignment_expression_left_post(&self) {
        self.instruction_emitter.emit_push("x0");
    }

    pub fn generate_assignment_expression_right_post(&self) {
        self.instruction_emitter.emit_pop("x1");
        self.instruction_emitter.emit_store("x0", "x1");
    }

    pub fn generate_binary_expression_right_post(&self) {
        self.instruction_emitter.emit_push("x0");
    }

    pub fn generate_binary_expression_left_post(&self) {
        self.instruction_emitter.emit_pop("x1");
    }

    pub fn generate_equal_expression_post(&self) {
        self.instruction_emitter.emit_conditional_set("eq");
    }

    pub fn generate_not_equal_expression_post(&self) {
        self.instruction_emitter.emit_conditional_set("ne");
    }

    pub fn generate_less_than_expression_post(&self) {
        self.instruction_emitter.emit_conditional_set("lt");
    }

    pub fn generate_less_than_or_equal_expression_post(&self) {
        self.instruction_emitter.emit_conditional_set("le");
    }

    pub fn generate_add_expression_post(&self) {
        self.instruction_emitter
            .emit_add_registers("x0", "x1", "x0");
    }

    pub fn generate_subtract_expression_post(&self) {
        self.instruction_emitter.emit_subtract("x0", "x1", "x0");
    }

    pub fn generate_multiply_expression_post(&self) {
        self.instruction_emitter.emit_multiply("x0", "x1", "x0");
    }

    pub fn generate_divide_expression_post(&self) {
        self.instruction_emitter.emit_divide("x0", "x1", "x0");
    }

    pub fn generate_negate_expression_post(&self) {
        self.instruction_emitter.emit_negate("x0", "x0");
    }

    pub fn generate_address_of_variable(
        &self,
        variable: &VariableReferenceExpression,
        context: &CodeGenerationContext,
    ) {
        // TODO: Emit an error if the variable is not found, instead of panicking.
        self.instruction_emitter
            .emit_move_signed_immediate_to_register(
                context
                    .get_local_variables()
                    .get(variable.get_name())
                    .unwrap()
                    .get_offset(),
                "x0",
            );

        self.instruction_emitter
            .emit_add_registers("fp", "x0", "x0");
    }

    pub fn generate_dereference_expression_post(&self) {
        self.instruction_emitter.emit_load("x0", "x0");
    }

    pub fn generate_variable_expression(
        &self,
        variable: &VariableReferenceExpression,
        context: &CodeGenerationContext,
    ) {
        // TODO: Emit an error if the variable is not found, instead of panicking.
        self.instruction_emitter.emit_load_offset(
            "fp",
            context
                .get_local_variables()
                .get(variable.get_name())
                .unwrap()
                .get_offset(),
            "x0",
        );
    }

    pub fn generate_number_expression(&self, number: &NumberExpression) {
        self.instruction_emitter
            .emit_move_unsigned_immediate_to_register(number.get_value(), "x0");
    }

    pub fn generate_function_call_expression_argument_push(&self) {
        self.instruction_emitter.emit_push("x0");
    }

    pub fn generate_function_call_expression_argument_move_to_register(
        &self,
        argument_index: usize,
    ) {
        self.instruction_emitter.emit_pop(
            self.instruction_emitter
                .get_function_parameter_register(argument_index),
        );
    }

    pub fn generate_function_call_expression_call(&self, expression: &FunctionCallExpression) {
        // TODO: This logic is only relevant to macOS.
        // This would need to be abstracted somehow when adding support
        // for other platforms.
        let function_name = format!("_{}", expression.get_name());
        self.instruction_emitter.emit_branch_link(&function_name);
    }
}
