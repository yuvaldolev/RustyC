use std::collections::HashMap;

use rustyc_ast::{BinaryOperator, Expression, ExpressionKind, UnaryOperator};
use rustyc_diagnostics::Diagnostic;

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter, variable_properties::VariableProperties,
};

pub struct ExpressionGenerator<'ast> {
    expression: &'ast Expression,
    local_variables: &'ast HashMap<String, VariableProperties>,
    instruction_emitter: Aarch64InstructionEmitter,
}

impl<'ast> ExpressionGenerator<'ast> {
    pub fn new(
        expression: &'ast Expression,
        local_variables: &'ast HashMap<String, VariableProperties>,
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
                self.generate_assignment_expression(left, right)?
            }
            ExpressionKind::Binary(operator, left, right) => {
                self.generate_binary_expression(operator, left, right)?
            }
            ExpressionKind::Unary(operator, right) => {
                self.generate_unary_expression(operator, right)?
            }
            ExpressionKind::Variable(variable) => self.generate_variable_expression(variable),
            ExpressionKind::Number(number) => self.generate_number_expression(*number),
        }

        Ok(())
    }

    fn generate_assignment_expression(
        &self,
        left: &Expression,
        right: &'ast Expression,
    ) -> rustyc_diagnostics::Result<()> {
        let ExpressionKind::Variable(variable) = left.get_kind() else {
            return Err(Diagnostic::new_error(
                rustyc_diagnostics::Error::InvalidAssignmentExpression,
                left.get_span().clone(),
            ));
        };

        let right_expression_generator = Self::new(right, self.local_variables);
        right_expression_generator.generate()?;

        self.instruction_emitter
            .emit_variable_write(self.local_variables.get(variable).unwrap());

        Ok(())
    }

    fn generate_binary_expression(
        &self,
        operator: &BinaryOperator,
        left: &'ast Expression,
        right: &'ast Expression,
    ) -> rustyc_diagnostics::Result<()> {
        let right_expression_generator = Self::new(right, self.local_variables);
        right_expression_generator.generate()?;
        self.instruction_emitter.emit_push("x0");

        let left_expression_generator = Self::new(left, self.local_variables);
        left_expression_generator.generate()?;

        self.instruction_emitter.emit_pop("x1");

        match operator {
            BinaryOperator::Equal => self.instruction_emitter.emit_comparison("eq"),
            BinaryOperator::NotEqual => self.instruction_emitter.emit_comparison("ne"),
            BinaryOperator::LessThan => self.instruction_emitter.emit_comparison("lt"),
            BinaryOperator::LessThanOrEqual => self.instruction_emitter.emit_comparison("le"),
            BinaryOperator::Add => self.instruction_emitter.emit_add("x0", "x1", "x0"),
            BinaryOperator::Subtract => self.instruction_emitter.emit_subtract("x0", "x1", "x0"),
            BinaryOperator::Multiply => self.instruction_emitter.emit_multiply("x0", "x1", "x0"),
            BinaryOperator::Divide => self.instruction_emitter.emit_divide("x0", "x1", "x0"),
        }

        Ok(())
    }

    fn generate_unary_expression(
        &self,
        operator: &UnaryOperator,
        right: &'ast Expression,
    ) -> rustyc_diagnostics::Result<()> {
        let right_expression_generator = Self::new(right, self.local_variables);
        right_expression_generator.generate()?;

        match operator {
            UnaryOperator::Negate => self.instruction_emitter.emit_negate("x0", "x0"),
        }

        Ok(())
    }

    fn generate_variable_expression(&self, variable: &str) {
        self.instruction_emitter
            .emit_variable_read(self.local_variables.get(variable).unwrap())
    }

    fn generate_number_expression(&self, number: u64) {
        self.instruction_emitter
            .emit_move(format!("#{number}").as_str(), "x0");
    }
}
