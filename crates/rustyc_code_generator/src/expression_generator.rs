use std::{collections::HashMap, rc::Rc};

use rustyc_diagnostics::Diagnostic;
use rustyc_hir::{BinaryOperator, Expression, ExpressionKind, UnaryOperator};

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
            ExpressionKind::Assignment(left, right) => {
                self.generate_assignment_expression(Rc::clone(left), Rc::clone(right))?
            }
            ExpressionKind::Binary(operator, left, right) => {
                self.generate_binary_expression(operator, Rc::clone(left), Rc::clone(right))?
            }
            ExpressionKind::Unary(operator, right) => {
                self.generate_unary_expression(operator, Rc::clone(right))?
            }
            ExpressionKind::Variable(variable) => self.generate_variable_expression(variable),
            ExpressionKind::Number(number) => self.generate_number_expression(*number),
            ExpressionKind::FunctionCall(name, arguments) => {
                self.generate_function_call_expression(name, arguments)?
            }
        }

        Ok(())
    }

    fn generate_assignment_expression(
        &self,
        left: Rc<Expression>,
        right: Rc<Expression>,
    ) -> rustyc_diagnostics::Result<()> {
        self.generate_address_of(left)?;
        self.instruction_emitter.emit_push("x0");

        let right_expression_generator = Self::new(right, Rc::clone(&self.local_variables));
        right_expression_generator.generate()?;

        self.instruction_emitter.emit_pop("x1");

        self.instruction_emitter.emit_store("x0", "x1");

        Ok(())
    }

    fn generate_binary_expression(
        &self,
        operator: &BinaryOperator,
        left: Rc<Expression>,
        right: Rc<Expression>,
    ) -> rustyc_diagnostics::Result<()> {
        let right_expression_generator = Self::new(right, Rc::clone(&self.local_variables));
        right_expression_generator.generate()?;
        self.instruction_emitter.emit_push("x0");

        let left_expression_generator = Self::new(left, Rc::clone(&self.local_variables));
        left_expression_generator.generate()?;

        self.instruction_emitter.emit_pop("x1");

        match operator {
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
        operator: &UnaryOperator,
        right: Rc<Expression>,
    ) -> rustyc_diagnostics::Result<()> {
        match operator {
            UnaryOperator::Negate => self.generate_negate(right)?,
            UnaryOperator::AddressOf => self.generate_address_of(right)?,
            UnaryOperator::Dereference => self.generate_dereference(right)?,
        }

        Ok(())
    }

    fn generate_variable_expression(&self, variable: &str) {
        // TODO: Emit an error if the variable is not found, instead of panicking.
        self.instruction_emitter.emit_load_offset(
            "fp",
            self.local_variables.get(variable).unwrap().get_offset(),
            "x0",
        );
    }

    fn generate_number_expression(&self, number: u64) {
        self.instruction_emitter
            .emit_move_registers(format!("#{number}").as_str(), "x0");
    }

    fn generate_function_call_expression(
        &self,
        name: &str,
        arguments: &[Rc<Expression>],
    ) -> rustyc_diagnostics::Result<()> {
        for argument in arguments.iter() {
            let argument_expression_generator =
                Self::new(Rc::clone(argument), Rc::clone(&self.local_variables));
            argument_expression_generator.generate()?;
            self.instruction_emitter.emit_push("x0");
        }

        for argument_index in (0..arguments.len()).rev() {
            self.instruction_emitter.emit_pop(
                self.instruction_emitter
                    .get_function_parameter_register(argument_index),
            );
        }

        // TODO: This logic is only relevant to macOS.
        // This would need to be abstracted somehow when adding support
        // for other platforms.
        let function_name = format!("_{name}");
        self.instruction_emitter.emit_branch_link(&function_name);

        Ok(())
    }

    fn generate_negate(&self, right: Rc<Expression>) -> rustyc_diagnostics::Result<()> {
        let right_expression_generator = Self::new(right, Rc::clone(&self.local_variables));
        right_expression_generator.generate()?;

        self.instruction_emitter.emit_negate("x0", "x0");

        Ok(())
    }

    fn generate_address_of(&self, right: Rc<Expression>) -> rustyc_diagnostics::Result<()> {
        match right.get_kind() {
            ExpressionKind::Variable(variable) => self.generate_address_of_variable(variable),
            ExpressionKind::Unary(UnaryOperator::Dereference, right) => {
                self.generate_address_of_dereference(Rc::clone(right))?
            }
            _ => {
                return Err(Diagnostic::new_error(
                    rustyc_diagnostics::Error::NotAnLvalue,
                    right.get_span().clone(),
                ))
            }
        }

        Ok(())
    }

    fn generate_dereference(&self, right: Rc<Expression>) -> rustyc_diagnostics::Result<()> {
        let right_expression_generator = Self::new(right, Rc::clone(&self.local_variables));
        right_expression_generator.generate()?;

        self.instruction_emitter.emit_load("x0", "x0");

        Ok(())
    }

    fn generate_address_of_variable(&self, variable: &str) {
        // TODO: Emit an error if the variable is not found, instead of panicking.
        self.instruction_emitter
            .emit_move_signed_immediate_to_register(
                self.local_variables.get(variable).unwrap().get_offset(),
                "x0",
            );

        self.instruction_emitter
            .emit_add_registers("fp", "x0", "x0");
    }

    fn generate_address_of_dereference(
        &self,
        right: Rc<Expression>,
    ) -> rustyc_diagnostics::Result<()> {
        let right_expression_generator = Self::new(right, Rc::clone(&self.local_variables));
        right_expression_generator.generate()?;

        Ok(())
    }
}
