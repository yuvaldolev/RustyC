use std::collections::HashMap;

use rustyc_ast::{
    BinaryOperator, Block, Expression, ExpressionKind, FunctionItem, Item, ItemKind, Statement,
    StatementKind, UnaryOperator,
};
use rustyc_diagnostics::Diagnostic;

use crate::{function_properties::FunctionProperties, variable_properties::VariableProperties};

pub struct CodeGenerator {
    ast: Item,
}

impl CodeGenerator {
    pub fn new(ast: Item) -> Self {
        Self { ast }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        println!(".text");
        println!();

        Self::generate_item(&self.ast)?;

        Ok(())
    }

    fn generate_item(item: &Item) -> rustyc_diagnostics::Result<()> {
        match item.get_kind() {
            ItemKind::Function(function) => Self::generate_function(function)?,
        }

        Ok(())
    }

    fn generate_function(function: &FunctionItem) -> rustyc_diagnostics::Result<()> {
        // TODO: In the future the function name will be parsed into the `FunctionItem`
        // struct and thus should be removed from the `FunctionProperties` struct.
        let function_properties = FunctionProperties::new(String::from("_main"), function);

        Self::generate_function_prologue(&function_properties);
        Self::generate_block(
            function.get_body(),
            function_properties.get_local_variables(),
        )?;
        Self::generate_function_epilogue();

        Ok(())
    }

    fn generate_block(
        block: &Block,
        local_variables: &HashMap<String, VariableProperties>,
    ) -> rustyc_diagnostics::Result<()> {
        for statement in block.get_statements().iter() {
            Self::generate_statement(statement, local_variables)?;
        }

        Ok(())
    }

    fn generate_statement(
        statement: &Statement,
        local_variables: &HashMap<String, VariableProperties>,
    ) -> rustyc_diagnostics::Result<()> {
        match statement.get_kind() {
            StatementKind::Expression(expression) => {
                Self::generate_expression(expression, local_variables)?;
            }
        }

        Ok(())
    }

    fn generate_expression(
        expression: &Expression,
        local_variables: &HashMap<String, VariableProperties>,
    ) -> rustyc_diagnostics::Result<()> {
        match expression.get_kind() {
            ExpressionKind::Assignment(left, right) => {
                Self::generate_assignment_expression(left, right, local_variables)?
            }
            ExpressionKind::Binary(operator, left, right) => {
                Self::generate_binary_expression(operator, left, right, local_variables)?
            }
            ExpressionKind::Unary(operator, right) => {
                Self::generate_unary_expression(operator, right, local_variables)?
            }
            ExpressionKind::Variable(variable) => {
                Self::generate_variable_expression(variable, local_variables)
            }
            ExpressionKind::Number(number) => Self::generate_number_expression(*number),
        }

        Ok(())
    }

    fn generate_assignment_expression(
        left: &Expression,
        right: &Expression,
        local_variables: &HashMap<String, VariableProperties>,
    ) -> rustyc_diagnostics::Result<()> {
        let ExpressionKind::Variable(variable) = left.get_kind() else {
            return Err(Diagnostic::new_error(
                rustyc_diagnostics::Error::InvalidAssignmentExpression,
                left.get_span().clone(),
            ));
        };

        Self::generate_expression(right, local_variables)?;
        Self::generate_variable_write(local_variables.get(variable).unwrap());

        Ok(())
    }

    fn generate_binary_expression(
        operator: &BinaryOperator,
        left: &Expression,
        right: &Expression,
        local_variables: &HashMap<String, VariableProperties>,
    ) -> rustyc_diagnostics::Result<()> {
        Self::generate_expression(right, local_variables)?;
        Self::generate_push("x0");

        Self::generate_expression(left, local_variables)?;

        Self::generate_pop("x1");

        match operator {
            BinaryOperator::Equal => Self::generate_comparison("eq"),
            BinaryOperator::NotEqual => Self::generate_comparison("ne"),
            BinaryOperator::LessThan => Self::generate_comparison("lt"),
            BinaryOperator::LessThanOrEqual => Self::generate_comparison("le"),
            BinaryOperator::Add => Self::emit_instruction("add x0, x0, x1"),
            BinaryOperator::Subtract => Self::emit_instruction("sub x0, x0, x1"),
            BinaryOperator::Multiply => Self::emit_instruction("mul x0, x0, x1"),
            BinaryOperator::Divide => Self::emit_instruction("sdiv x0, x0, x1"),
        }

        Ok(())
    }

    fn generate_unary_expression(
        operator: &UnaryOperator,
        right: &Expression,
        local_variables: &HashMap<String, VariableProperties>,
    ) -> rustyc_diagnostics::Result<()> {
        Self::generate_expression(right, local_variables)?;

        match operator {
            UnaryOperator::Negate => Self::emit_instruction("neg x0, x0"),
        }

        Ok(())
    }

    fn generate_variable_expression(
        variable: &str,
        local_variables: &HashMap<String, VariableProperties>,
    ) {
        Self::generate_variable_read(local_variables.get(variable).unwrap())
    }

    fn generate_number_expression(number: u64) {
        Self::emit_instruction(format!("mov x0, #{}", number).as_str())
    }

    fn generate_function_prologue(properties: &FunctionProperties) {
        println!(".global {}", properties.get_name());
        println!("{}:", properties.get_name());

        Self::generate_push("fp");
        Self::emit_instruction("mov fp, sp");
        Self::emit_instruction(format!("sub sp, sp, {}", properties.get_stack_size()).as_str());
    }

    fn generate_function_epilogue() {
        Self::emit_instruction("mov sp, fp");
        Self::generate_pop("fp");
        Self::emit_instruction("ret");
    }

    fn generate_comparison(condition: &str) {
        Self::emit_instruction("cmp x0, x1");
        Self::emit_instruction(format!("cset x0, {condition}").as_str());
    }

    fn generate_variable_read(variable: &VariableProperties) {
        Self::emit_instruction(format!("ldr x0, [fp, #{}]", variable.get_offset()).as_str());
    }

    fn generate_variable_write(variable: &VariableProperties) {
        Self::emit_instruction(format!("str x0, [fp, #{}]", variable.get_offset()).as_str());
    }

    fn generate_push(register: &str) {
        Self::generate_push_pair(register, "xzr");
    }

    fn generate_push_pair(register1: &str, register2: &str) {
        Self::emit_instruction(format!("stp {register1}, {register2}, [sp, #-0x10]!").as_str());
    }

    fn generate_pop(register: &str) {
        Self::generate_pop_pair(register, "xzr");
    }

    fn generate_pop_pair(register1: &str, register2: &str) {
        Self::emit_instruction(format!("ldp {register1}, {register2}, [sp], #0x10").as_str());
    }

    fn emit_instruction(instruction: &str) {
        println!("  {instruction}");
    }
}
