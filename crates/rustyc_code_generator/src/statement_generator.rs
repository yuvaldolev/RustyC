use std::collections::HashMap;

use rustyc_ast::{Expression, Statement, StatementKind};

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter, block_generator::BlockGenerator,
    expression_generator::ExpressionGenerator, variable_properties::VariableProperties,
};

pub struct StatementGenerator<'ast> {
    statement: &'ast Statement,
    local_variables: &'ast HashMap<String, VariableProperties>,
    instruction_emitter: Aarch64InstructionEmitter,
}

impl<'ast> StatementGenerator<'ast> {
    pub fn new(
        statement: &'ast Statement,
        local_variables: &'ast HashMap<String, VariableProperties>,
    ) -> Self {
        Self {
            statement,
            local_variables,
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        match self.statement.get_kind() {
            StatementKind::Compound(block) => {
                let block_generator = BlockGenerator::new(block, self.local_variables);
                block_generator.generate()?;
            }
            StatementKind::Return(expression) => {
                self.generate_expression(expression)?;
                self.instruction_emitter.emit_branch(".L.return");
            }
            StatementKind::Expression(expression) => self.generate_expression(expression)?,
        }

        Ok(())
    }

    fn generate_expression(&self, expression: &Expression) -> rustyc_diagnostics::Result<()> {
        let expression_generator = ExpressionGenerator::new(expression, self.local_variables);
        expression_generator.generate()
    }
}
