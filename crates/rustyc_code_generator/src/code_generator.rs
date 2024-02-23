use rustyc_ast::{Node, NodeKind};
use rustyc_diagnostics::Diagnostic;

pub struct CodeGenerator {
    ast: Vec<Box<Node>>,
}

impl CodeGenerator {
    pub fn new(ast: Vec<Box<Node>>) -> Self {
        Self { ast }
    }

    pub fn generate(&self) -> rustyc_diagnostics::Result<()> {
        Self::generate_prologue();

        for statement in self.ast.iter() {
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

    fn generate_statement(node: &Node) -> rustyc_diagnostics::Result<()> {
        match node.get_kind() {
            NodeKind::ExpressionStatement => {
                let left = node.get_left().ok_or(Diagnostic::new_error(
                    rustyc_diagnostics::Error::InvalidExpressionStatement,
                    node.get_span().clone(),
                ))?;
                Self::generate_expression(left)?;
            }
            _ => {
                return Err(Diagnostic::new_error(
                    rustyc_diagnostics::Error::InvalidStatement,
                    node.get_span().clone(),
                ));
            }
        }

        Ok(())
    }

    fn generate_epilogue() {
        Self::emit_instruction("mov sp, fp");
        Self::generate_pop("fp");
        Self::emit_instruction("ret");
    }

    fn generate_expression(node: &Node) -> rustyc_diagnostics::Result<()> {
        match node.get_kind() {
            NodeKind::Number(number) => {
                Self::emit_instruction(format!("mov x0, #{}", number).as_str());
                return Ok(());
            }
            NodeKind::Variable(variable) => {
                Self::generate_variable_read(*variable);
                return Ok(());
            }
            NodeKind::Negation => {
                let left = node.get_left().ok_or(Diagnostic::new_error(
                    rustyc_diagnostics::Error::InvalidNegationExpression,
                    node.get_span().clone(),
                ))?;
                Self::generate_expression(left)?;
                Self::emit_instruction("neg x0, x0");
                return Ok(());
            }
            NodeKind::Assignment => {
                let left = node.get_left().ok_or(Diagnostic::new_error(
                    rustyc_diagnostics::Error::InvalidAssignmentExpression,
                    node.get_span().clone(),
                ))?;
                let NodeKind::Variable(left_variable) = left.get_kind() else {
                    return Err(Diagnostic::new_error(
                        rustyc_diagnostics::Error::InvalidAssignmentExpression,
                        left.get_span().clone(),
                    ));
                };

                let right = node.get_right().ok_or(Diagnostic::new_error(
                    rustyc_diagnostics::Error::InvalidAssignmentExpression,
                    node.get_span().clone(),
                ))?;
                Self::generate_expression(right)?;

                Self::generate_variable_write(*left_variable);

                return Ok(());
            }
            _ => {}
        }

        let right = node.get_right().ok_or(Diagnostic::new_error(
            rustyc_diagnostics::Error::InvalidExpression,
            node.get_span().clone(),
        ))?;
        Self::generate_expression(right)?;

        Self::generate_push("x0");

        let left = node.get_left().ok_or(Diagnostic::new_error(
            rustyc_diagnostics::Error::InvalidExpression,
            node.get_span().clone(),
        ))?;
        Self::generate_expression(left)?;

        Self::generate_pop("x1");

        match node.get_kind() {
            NodeKind::Addition => Self::emit_instruction("add x0, x0, x1"),
            NodeKind::Subtraction => Self::emit_instruction("sub x0, x0, x1"),
            NodeKind::Multiplication => Self::emit_instruction("mul x0, x0, x1"),
            NodeKind::Division => Self::emit_instruction("sdiv x0, x0, x1"),
            NodeKind::Equality => Self::generate_comparison("eq"),
            NodeKind::NotEqual => Self::generate_comparison("ne"),
            NodeKind::LessThan => Self::generate_comparison("lt"),
            NodeKind::LessThanOrEqual => Self::generate_comparison("le"),
            _ => {
                return Err(Diagnostic::new_error(
                    rustyc_diagnostics::Error::InvalidExpression,
                    node.get_span().clone(),
                ));
            }
        }

        Ok(())
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
