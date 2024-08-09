use std::{collections::HashMap, rc::Rc};

use rustyc_diagnostics::Diagnostic;
use rustyc_hir::expressions::{
    AssignmentExpression, BinaryExpression, BinaryOperator, Expression, ExpressionKind,
    FunctionCallExpression, NumberExpression, UnaryExpression, UnaryOperator, VariableExpression,
};

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter, variable_properties::VariableProperties,
};

pub struct ExpressionGenerator {
    expression: Rc<Expression>,
    local_variables: Rc<HashMap<String, VariableProperties>>,
    instruction_emitter: Aarch64InstructionEmitter,
}

impl ExpressionGenerator {
    pub fn new(
        expression: Rc<Expression>,
        local_variables: Rc<HashMap<String, VariableProperties>>,
    ) -> Self {
        Self {
            expression,
            local_variables,
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        match self.expression.get_kind() {
            ExpressionKind::Assignment(expression) => {
                self.generate_assignment_expression(expression)?
            }
            ExpressionKind::Binary(expression) => self.generate_binary_expression(expression)?,
            ExpressionKind::Unary(expression) => self.generate_unary_expression(expression)?,
            ExpressionKind::Variable(expression) => self.generate_variable_expression(expression),
            ExpressionKind::Number(expression) => self.generate_number_expression(expression),
            ExpressionKind::FunctionCall(expression) => {
                self.generate_function_call_expression(expression)?
            }
        }

        Ok(())
    }

    fn generate_assignment_expression(
        &self,
        expression: &AssignmentExpression,
    ) -> rustyc_diagnostics::Result<()> {
        self.generate_address_of(expression.get_left())?;
        self.instruction_emitter.emit_push("x0");

        self.generate_expression(expression.get_right())?;

        self.instruction_emitter.emit_pop("x1");

        self.instruction_emitter.emit_store("x0", "x1");

        Ok(())
    }

    fn generate_binary_expression(
        &self,
        expression: &BinaryExpression,
    ) -> rustyc_diagnostics::Result<()> {
        self.generate_expression(expression.get_right())?;
        self.instruction_emitter.emit_push("x0");

        self.generate_expression(expression.get_left())?;

        self.instruction_emitter.emit_pop("x1");

        match expression.get_operator() {
            BinaryOperator::Equal => self.instruction_emitter.emit_conditional_set("eq"),
            BinaryOperator::NotEqual => self.instruction_emitter.emit_conditional_set("ne"),
            BinaryOperator::LessThan => self.instruction_emitter.emit_conditional_set("lt"),
            BinaryOperator::LessThanOrEqual => self.instruction_emitter.emit_conditional_set("le"),
            BinaryOperator::Add => self
                .instruction_emitter
                .emit_add_registers("x0", "x1", "x0"),
            BinaryOperator::Subtract => self.instruction_emitter.emit_subtract("x0", "x1", "x0"),
            BinaryOperator::Multiply => self.instruction_emitter.emit_multiply("x0", "x1", "x0"),
            BinaryOperator::Divide => self.instruction_emitter.emit_divide("x0", "x1", "x0"),
        }

        Ok(())
    }

    fn generate_unary_expression(
        &self,
        expression: &UnaryExpression,
    ) -> rustyc_diagnostics::Result<()> {
        match expression.get_operator() {
            UnaryOperator::Negate => self.generate_negate(expression.get_operand())?,
            UnaryOperator::AddressOf => self.generate_address_of(expression.get_operand())?,
            UnaryOperator::Dereference => self.generate_dereference(expression.get_operand())?,
        }

        Ok(())
    }

    fn generate_variable_expression(&self, expression: &VariableExpression) {
        // TODO: Emit an error if the variable is not found, instead of panicking.
        self.instruction_emitter.emit_load_offset(
            "fp",
            self.local_variables
                .get(expression.get_name())
                .unwrap()
                .get_offset(),
            "x0",
        );
    }

    fn generate_number_expression(&self, expression: &NumberExpression) {
        self.instruction_emitter
            .emit_move_unsigned_immediate_to_register(expression.get_value(), "x0");
    }

    fn generate_function_call_expression(
        &self,
        expression: &FunctionCallExpression,
    ) -> rustyc_diagnostics::Result<()> {
        for argument in expression.get_arguments().iter() {
            self.generate_expression(Rc::clone(argument))?;
            self.instruction_emitter.emit_push("x0");
        }

        for argument_index in (0..expression.get_arguments().len()).rev() {
            self.instruction_emitter.emit_pop(
                self.instruction_emitter
                    .get_function_parameter_register(argument_index),
            );
        }

        // TODO: This logic is only relevant to macOS.
        // This would need to be abstracted somehow when adding support
        // for other platforms.
        let function_name = format!("_{}", expression.get_name());
        self.instruction_emitter.emit_branch_link(&function_name);

        Ok(())
    }

    fn generate_negate(&self, expression: Rc<Expression>) -> rustyc_diagnostics::Result<()> {
        self.generate_expression(expression)?;
        self.instruction_emitter.emit_negate("x0", "x0");

        Ok(())
    }

    fn generate_address_of(&self, expression: Rc<Expression>) -> rustyc_diagnostics::Result<()> {
        match expression.get_kind() {
            ExpressionKind::Variable(expression) => self.generate_address_of_variable(expression),
            ExpressionKind::Unary(expression)
                if UnaryOperator::Dereference == *expression.get_operator() =>
            {
                self.generate_expression(expression.get_operand())?
            }
            _ => {
                return Err(Diagnostic::new_error(
                    rustyc_diagnostics::Error::NotAnLvalue,
                    expression.get_span().clone(),
                ))
            }
        }

        Ok(())
    }

    fn generate_dereference(&self, expression: Rc<Expression>) -> rustyc_diagnostics::Result<()> {
        self.generate_expression(expression)?;
        self.instruction_emitter.emit_load("x0", "x0");

        Ok(())
    }

    fn generate_address_of_variable(&self, expression: &VariableExpression) {
        // TODO: Emit an error if the variable is not found, instead of panicking.
        self.instruction_emitter
            .emit_move_signed_immediate_to_register(
                self.local_variables
                    .get(expression.get_name())
                    .unwrap()
                    .get_offset(),
                "x0",
            );

        self.instruction_emitter
            .emit_add_registers("fp", "x0", "x0");
    }

    fn generate_expression(&self, expression: Rc<Expression>) -> rustyc_diagnostics::Result<()> {
        let expression_generator = Self::new(expression, Rc::clone(&self.local_variables));
        expression_generator.generate()
    }
}
