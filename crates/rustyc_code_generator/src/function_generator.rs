use rustyc_ast::{
    BinaryOperator, Block, Expression, ExpressionKind, FunctionItem, Statement, StatementKind,
    UnaryOperator,
};
use rustyc_diagnostics::Diagnostic;

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter, function_properties::FunctionProperties,
};

pub struct FunctionGenerator {
    item: FunctionItem,
    properties: FunctionProperties,
    instruction_emitter: Aarch64InstructionEmitter,
}

impl FunctionGenerator {
    pub fn new(name: String, item: FunctionItem) -> Self {
        // TODO: In the future the function name will be parsed into the `FunctionItem`
        // struct and thus should be removed from the `FunctionProperties` struct.
        let properties = FunctionProperties::new(name, &item);

        Self {
            item,
            properties,
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        self.generate_prologue();
        self.generate_block(self.item.get_body())?;
        self.generate_epilogue();

        Ok(())
    }

    fn generate_prologue(&self) {
        self.instruction_emitter
            .emit_global(self.properties.get_name());
        self.instruction_emitter
            .emit_label(self.properties.get_name());

        self.instruction_emitter.emit_push("fp");
        self.instruction_emitter.emit_move("sp", "fp");
        self.instruction_emitter.emit_subtract(
            "sp",
            self.properties.get_stack_size().to_string().as_str(),
            "sp",
        );
    }

    fn generate_epilogue(&self) {
        self.instruction_emitter.emit_label(".L.return");

        self.instruction_emitter.emit_move("fp", "sp");
        self.instruction_emitter.emit_pop("fp");
        self.instruction_emitter.emit_return();
    }

    fn generate_block(&self, block: &Block) -> rustyc_diagnostics::Result<()> {
        for statement in block.get_statements().iter() {
            self.generate_statement(statement)?;
        }

        Ok(())
    }

    fn generate_statement(&self, statement: &Statement) -> rustyc_diagnostics::Result<()> {
        match statement.get_kind() {
            StatementKind::Compound(block) => {
                self.generate_block(block)?;
            }
            StatementKind::Return(expression) => {
                self.generate_expression(expression)?;
                self.instruction_emitter.emit_branch(".L.return");
            }
            StatementKind::Expression(expression) => {
                self.generate_expression(expression)?;
            }
        }

        Ok(())
    }

    fn generate_expression(&self, expression: &Expression) -> rustyc_diagnostics::Result<()> {
        match expression.get_kind() {
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
        right: &Expression,
    ) -> rustyc_diagnostics::Result<()> {
        let ExpressionKind::Variable(variable) = left.get_kind() else {
            return Err(Diagnostic::new_error(
                rustyc_diagnostics::Error::InvalidAssignmentExpression,
                left.get_span().clone(),
            ));
        };

        self.generate_expression(right)?;
        self.instruction_emitter
            .emit_variable_write(self.properties.get_local_variables().get(variable).unwrap());

        Ok(())
    }

    fn generate_binary_expression(
        &self,
        operator: &BinaryOperator,
        left: &Expression,
        right: &Expression,
    ) -> rustyc_diagnostics::Result<()> {
        self.generate_expression(right)?;
        self.instruction_emitter.emit_push("x0");

        self.generate_expression(left)?;

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
        right: &Expression,
    ) -> rustyc_diagnostics::Result<()> {
        self.generate_expression(right)?;

        match operator {
            UnaryOperator::Negate => self.instruction_emitter.emit_negate("x0", "x0"),
        }

        Ok(())
    }

    fn generate_variable_expression(&self, variable: &str) {
        self.instruction_emitter
            .emit_variable_read(self.properties.get_local_variables().get(variable).unwrap())
    }

    fn generate_number_expression(&self, number: u64) {
        self.instruction_emitter
            .emit_move(format!("#{number}").as_str(), "x0");
    }
}
