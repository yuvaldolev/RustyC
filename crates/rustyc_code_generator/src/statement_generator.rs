use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rustyc_hir::{
    expressions::Expression,
    statements::{
        CompoundStatement, ExpressionStatement, IfStatement, LoopStatement, ReturnStatement,
        Statement, StatementKind,
    },
    Block,
};

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter, block_generator::BlockGenerator,
    expression_generator::ExpressionGenerator, label_allocator::LabelAllocator,
    variable_properties::VariableProperties,
};

pub struct StatementGenerator {
    statement: Rc<Statement>,
    local_variables: Rc<HashMap<String, VariableProperties>>,
    label_allocator: Rc<RefCell<LabelAllocator>>,
    instruction_emitter: Aarch64InstructionEmitter,
}

impl StatementGenerator {
    pub fn new(
        statement: Rc<Statement>,
        local_variables: Rc<HashMap<String, VariableProperties>>,
        label_allocator: Rc<RefCell<LabelAllocator>>,
    ) -> Self {
        Self {
            statement,
            local_variables,
            label_allocator,
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        match self.statement.get_kind() {
            StatementKind::Return(statement) => self.generate_return_statement(statement),
            StatementKind::If(statement) => self.generate_if_statement(statement),
            StatementKind::Loop(statement) => self.generate_loop_statement(statement),
            StatementKind::Compound(statement) => self.generate_compound_statement(statement),
            StatementKind::Expression(statement) => self.generate_expression_statement(statement),
        }
    }

    fn generate_return_statement(
        &self,
        statement: &ReturnStatement,
    ) -> rustyc_diagnostics::Result<()> {
        self.generate_expression(statement.get_expression())?;
        self.instruction_emitter.emit_branch(
            self.label_allocator
                .borrow()
                .allocate_global("return")
                .as_str(),
        );

        Ok(())
    }

    fn generate_if_statement(&self, statement: &IfStatement) -> rustyc_diagnostics::Result<()> {
        let else_label = self.label_allocator.borrow_mut().allocate_unique("else");
        let end_label = self.label_allocator.borrow_mut().allocate_unique("end");

        self.generate_expression(statement.get_condition_expression())?;
        self.instruction_emitter.emit_comparison("x0", "#0");
        self.instruction_emitter.emit_branch_equals(&else_label);

        self.generate_statement(statement.get_then_statement())?;
        self.instruction_emitter.emit_branch(&end_label);

        self.instruction_emitter.emit_label(&else_label);
        statement
            .get_else_statement()
            .map(|value| self.generate_statement(value))
            .transpose()?;

        self.instruction_emitter.emit_label(&end_label);

        Ok(())
    }

    fn generate_loop_statement(&self, statement: &LoopStatement) -> rustyc_diagnostics::Result<()> {
        let begin_label = self.label_allocator.borrow_mut().allocate_unique("begin");
        let end_label = self.label_allocator.borrow_mut().allocate_unique("end");

        statement
            .get_initialization_statement()
            .map(|value| self.generate_statement(value))
            .transpose()?;

        self.instruction_emitter.emit_label(&begin_label);

        if let Some(expression) = statement.get_condition_expression() {
            self.generate_expression(expression)?;
            self.instruction_emitter.emit_comparison("x0", "#0");
            self.instruction_emitter.emit_branch_equals(&end_label);
        }

        self.generate_statement(statement.get_then_statement())?;

        statement
            .get_incrementation_expression()
            .map(|value| self.generate_expression(value))
            .transpose()?;

        self.instruction_emitter.emit_branch(&begin_label);

        self.instruction_emitter.emit_label(&end_label);

        Ok(())
    }

    fn generate_compound_statement(
        &self,
        statement: &CompoundStatement,
    ) -> rustyc_diagnostics::Result<()> {
        self.generate_block(statement.get_block())
    }

    fn generate_expression_statement(
        &self,
        statement: &ExpressionStatement,
    ) -> rustyc_diagnostics::Result<()> {
        self.generate_expression(statement.get_expression())
    }

    fn generate_statement(&self, statement: Rc<Statement>) -> rustyc_diagnostics::Result<()> {
        let statement_generator = Self::new(
            statement,
            Rc::clone(&self.local_variables),
            Rc::clone(&self.label_allocator),
        );
        statement_generator.generate()
    }

    fn generate_expression(&self, expression: Rc<Expression>) -> rustyc_diagnostics::Result<()> {
        let expression_generator =
            ExpressionGenerator::new(expression, Rc::clone(&self.local_variables));
        expression_generator.generate()
    }

    fn generate_block(&self, block: Rc<Block>) -> rustyc_diagnostics::Result<()> {
        let block_generator = BlockGenerator::new(
            block,
            Rc::clone(&self.local_variables),
            Rc::clone(&self.label_allocator),
        );
        block_generator.generate()
    }
}
