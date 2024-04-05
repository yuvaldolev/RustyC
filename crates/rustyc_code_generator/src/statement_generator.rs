use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rustyc_ast::{Block, Expression, Statement, StatementKind};

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
            StatementKind::Return(expression) => self.generate_return(Rc::clone(expression)),
            StatementKind::If(condition_expression, then_statement, else_statement) => self
                .generate_if(
                    Rc::clone(condition_expression),
                    Rc::clone(then_statement),
                    else_statement.as_ref().map(|value| Rc::clone(value)),
                ),
            StatementKind::For(
                initialization_statement,
                condition_expression,
                incrementation_expression,
                then_statement,
            ) => self.generate_for(
                Rc::clone(initialization_statement),
                condition_expression.as_ref().map(|value| Rc::clone(value)),
                incrementation_expression
                    .as_ref()
                    .map(|value| Rc::clone(value)),
                Rc::clone(then_statement),
            ),
            StatementKind::Compound(block) => self.generate_compound(Rc::clone(block)),
            StatementKind::Expression(expression) => {
                self.generate_expression(Rc::clone(expression))
            }
        }
    }

    fn generate_compound(&self, block: Rc<Block>) -> rustyc_diagnostics::Result<()> {
        let block_generator = BlockGenerator::new(
            block,
            Rc::clone(&self.local_variables),
            Rc::clone(&self.label_allocator),
        );
        block_generator.generate()
    }

    fn generate_return(&self, expression: Rc<Expression>) -> rustyc_diagnostics::Result<()> {
        self.generate_expression(expression)?;
        self.instruction_emitter.emit_branch(".L.return");

        Ok(())
    }

    fn generate_if(
        &self,
        condition_expression: Rc<Expression>,
        then_statement: Rc<Statement>,
        else_statement: Option<Rc<Statement>>,
    ) -> rustyc_diagnostics::Result<()> {
        let else_label = self.label_allocator.borrow_mut().allocate("else");
        let end_label = self.label_allocator.borrow_mut().allocate("end");

        self.generate_expression(condition_expression)?;
        self.instruction_emitter.emit_comparison("x0", "#0");
        self.instruction_emitter.emit_branch_equals(&else_label);

        let then_statement_generator = Self::new(
            then_statement,
            Rc::clone(&self.local_variables),
            Rc::clone(&self.label_allocator),
        );
        then_statement_generator.generate()?;
        self.instruction_emitter.emit_branch(&end_label);

        self.instruction_emitter.emit_label(&else_label);
        if let Some(statement) = else_statement {
            let else_statement_generator = Self::new(
                statement,
                Rc::clone(&self.local_variables),
                Rc::clone(&self.label_allocator),
            );
            else_statement_generator.generate()?;
        }

        self.instruction_emitter.emit_label(&end_label);

        Ok(())
    }

    fn generate_for(
        &self,
        initialization_statement: Rc<Statement>,
        condition_expression: Option<Rc<Expression>>,
        incrementation_expression: Option<Rc<Expression>>,
        then_statement: Rc<Statement>,
    ) -> rustyc_diagnostics::Result<()> {
        let begin_label = self.label_allocator.borrow_mut().allocate("begin");
        let end_label = self.label_allocator.borrow_mut().allocate("end");

        let initialization_statement_generator = Self::new(
            initialization_statement,
            Rc::clone(&self.local_variables),
            Rc::clone(&self.label_allocator),
        );
        initialization_statement_generator.generate()?;

        self.instruction_emitter.emit_label(&begin_label);

        if let Some(expression) = condition_expression {
            self.generate_expression(expression)?;
            self.instruction_emitter.emit_comparison("x0", "#0");
            self.instruction_emitter.emit_branch_equals(&end_label);
        }

        let then_statement_generator = Self::new(
            then_statement,
            Rc::clone(&self.local_variables),
            Rc::clone(&self.label_allocator),
        );
        then_statement_generator.generate()?;

        if let Some(expression) = incrementation_expression {
            self.generate_expression(expression)?;
        }

        self.instruction_emitter.emit_branch(&begin_label);

        self.instruction_emitter.emit_label(&end_label);

        Ok(())
    }

    fn generate_expression(&self, expression: Rc<Expression>) -> rustyc_diagnostics::Result<()> {
        let expression_generator =
            ExpressionGenerator::new(expression, Rc::clone(&self.local_variables));
        expression_generator.generate()
    }
}
