use rustyc_ast::{
    BinaryOperator, Block, Expression, ExpressionKind, Statement, StatementKind, UnaryOperator,
};
use rustyc_diagnostics::Diagnostic;

pub struct CodeGenerator {
    ast: Block,
}

impl CodeGenerator {
    pub fn new(ast: Block) -> Self {
        Self { ast }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        Self::generate_prologue();

        for statement in self.ast.get_statements().iter() {
            Self::generate_statement(statement)?;
        }

        Self::generate_epilogue();

        Ok(())
    }

    fn generate_prologue() {
        println!(".text");
        println!();
        println!(".global _main");
        println!("_main:");

        Self::generate_push("fp");
        Self::emit_instruction("mov fp, sp");
        Self::emit_instruction("sub sp, sp, #208");
    }

    fn generate_statement(statement: &Statement) -> rustyc_diagnostics::Result<()> {
        match statement.get_kind() {
            StatementKind::Expression(expression) => {
                Self::generate_expression(expression)?;
            }
        }

        Ok(())
    }

    fn generate_epilogue() {
        Self::emit_instruction("mov sp, fp");
        Self::generate_pop("fp");
        Self::emit_instruction("ret");
    }

    fn generate_expression(expression: &Expression) -> rustyc_diagnostics::Result<()> {
        match expression.get_kind() {
            ExpressionKind::Assignment(left, right) => {
                Self::generate_assignment_expression(left, right)?
            }
            ExpressionKind::Binary(operator, left, right) => {
                Self::generate_binary_expression(operator, left, right)?
            }
            ExpressionKind::Unary(operator, right) => {
                Self::generate_unary_expression(operator, right)?
            }
            ExpressionKind::Variable(variable) => Self::generate_variable_expression(*variable),
            ExpressionKind::Number(number) => Self::generate_number_expression(*number),
        }

        Ok(())
    }

    fn generate_assignment_expression(
        left: &Expression,
        right: &Expression,
    ) -> rustyc_diagnostics::Result<()> {
        let ExpressionKind::Variable(variable) = left.get_kind() else {
            return Err(Diagnostic::new_error(
                rustyc_diagnostics::Error::InvalidAssignmentExpression,
                left.get_span().clone(),
            ));
        };

        Self::generate_expression(right)?;
        Self::generate_variable_write(*variable);

        Ok(())
    }

    fn generate_binary_expression(
        operator: &BinaryOperator,
        left: &Expression,
        right: &Expression,
    ) -> rustyc_diagnostics::Result<()> {
        Self::generate_expression(right)?;
        Self::generate_push("x0");

        Self::generate_expression(left)?;

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
    ) -> rustyc_diagnostics::Result<()> {
        Self::generate_expression(right)?;

        match operator {
            UnaryOperator::Negate => Self::emit_instruction("neg x0, x0"),
        }

        Ok(())
    }

    fn generate_variable_expression(variable: char) {
        Self::generate_variable_read(variable)
    }

    fn generate_number_expression(number: u64) {
        Self::emit_instruction(format!("mov x0, #{}", number).as_str())
    }

    fn generate_comparison(condition: &str) {
        Self::emit_instruction("cmp x0, x1");
        Self::emit_instruction(format!("cset x0, {condition}").as_str());
    }

    fn generate_variable_read(variable: char) {
        Self::emit_instruction(
            format!(
                "ldr x0, [fp, #-{}]",
                Self::calculate_variable_offset(variable)
            )
            .as_str(),
        );
    }

    fn generate_variable_write(variable: char) {
        Self::emit_instruction(
            format!(
                "str x0, [fp, #-{}]",
                Self::calculate_variable_offset(variable)
            )
            .as_str(),
        );
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

    fn calculate_variable_offset(variable: char) -> isize {
        8 * (((variable as u8) - b'a' + 1) as isize)
    }
}
