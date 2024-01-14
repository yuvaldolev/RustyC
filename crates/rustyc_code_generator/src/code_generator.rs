use rustyc_ast::{Node, NodeKind};
use rustyc_diagnostics::Diagnostic;

pub struct CodeGenerator {
    ast: Box<Node>,
}

impl CodeGenerator {
    pub fn new(ast: Box<Node>) -> Self {
        Self { ast }
    }

    pub fn generate(&self) -> rustyc_diagnostics::Result<()> {
        Self::generate_prologue();
        Self::generate_expression(&self.ast)?;
        Self::generate_epilogue();

        Ok(())
    }

    fn generate_prologue() {
        println!(".text");
        println!();
        println!(".global _main");
        println!("_main:");
    }

    fn generate_expression(node: &Node) -> rustyc_diagnostics::Result<()> {
        if let NodeKind::Number(number) = node.get_kind() {
            Self::emit_instruction(format!("mov x0, #{}", number.get_value()).as_str());
            return Ok(());
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
            NodeKind::Add => Self::emit_instruction("add x0, x0, x1"),
            NodeKind::Subtract => Self::emit_instruction("sub x0, x0, x1"),
            NodeKind::Multiply => Self::emit_instruction("mul x0, x0, x1"),
            NodeKind::Divide => Self::emit_instruction("sdiv x0, x0, x1"),
            _ => {
                return Err(Diagnostic::new_error(
                    rustyc_diagnostics::Error::InvalidExpression,
                    node.get_span().clone(),
                ))
            }
        }

        Ok(())
    }

    fn generate_epilogue() {
        Self::emit_instruction("ret");
    }

    fn generate_push(register: &str) {
        Self::emit_instruction(format!("stp {register}, xzr, [sp, #-0x10]!").as_str());
    }

    fn generate_pop(register: &str) {
        Self::emit_instruction(format!("ldp {register}, xzr, [sp], #0x10").as_str());
    }

    fn emit_instruction(instruction: &str) {
        println!("  {instruction}");
    }
}
