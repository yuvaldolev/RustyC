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

    pub fn generate_if_statement_end(&self, end_label: &str) {
        self.instruction_emitter.emit_label(&end_label);
    }

    // fn generate_loop_statement(&self, statement: &LoopStatement) -> rustyc_diagnostics::Result<()> {
    //     let begin_label = self.label_allocator.borrow_mut().allocate_unique("begin");
    //     let end_label = self.label_allocator.borrow_mut().allocate_unique("end");
    //
    //     statement
    //         .get_initialization_statement()
    //         .map(|value| self.generate_statement(value))
    //         .transpose()?;
    //
    //     self.instruction_emitter.emit_label(&begin_label);
    //
    //     if let Some(expression) = statement.get_condition_expression() {
    //         self.generate_expression(expression)?;
    //         self.instruction_emitter.emit_comparison("x0", "#0");
    //         self.instruction_emitter.emit_branch_equals(&end_label);
    //     }
    //
    //     self.generate_statement(statement.get_then_statement())?;
    //
    //     statement
    //         .get_incrementation_expression()
    //         .map(|value| self.generate_expression(value))
    //         .transpose()?;
    //
    //     self.instruction_emitter.emit_branch(&begin_label);
    //
    //     self.instruction_emitter.emit_label(&end_label);
    //
    //     Ok(())
    // }
    //
    // fn generate_compound_statement(
    //     &self,
    //     statement: &CompoundStatement,
    // ) -> rustyc_diagnostics::Result<()> {
    //     self.generate_block(statement.get_block())
    // }
    //
    // fn generate_expression_statement(
    //     &self,
    //     statement: &ExpressionStatement,
    // ) -> rustyc_diagnostics::Result<()> {
    //     self.generate_expression(statement.get_expression())
    // }
    //
    // fn generate_statement(&self, statement: Rc<Statement>) -> rustyc_diagnostics::Result<()> {
    //     let statement_generator = Self::new(
    //         statement,
    //         Rc::clone(&self.local_variables),
    //         Rc::clone(&self.label_allocator),
    //     );
    //     statement_generator.generate()
    // }
    //
    // fn generate_expression(&self, expression: Rc<Expression>) -> rustyc_diagnostics::Result<()> {
    //     let expression_generator =
    //         ExpressionGenerator::new(expression, Rc::clone(&self.local_variables));
    //     expression_generator.generate()
    // }
    //
    // fn generate_block(&self, block: Rc<Block>) -> rustyc_diagnostics::Result<()> {
    //     let block_generator = BlockGenerator::new(
    //         block,
    //         Rc::clone(&self.local_variables),
    //         Rc::clone(&self.label_allocator),
    //     );
    //     block_generator.generate()
    // }
}
