use std::collections::HashMap;

use rustyc_ast::Block;

use crate::{statement_generator::StatementGenerator, variable_properties::VariableProperties};

pub struct BlockGenerator<'ast> {
    block: &'ast Block,
    local_variables: &'ast HashMap<String, VariableProperties>,
}

impl<'ast> BlockGenerator<'ast> {
    pub fn new(
        block: &'ast Block,
        local_variables: &'ast HashMap<String, VariableProperties>,
    ) -> Self {
        Self {
            block,
            local_variables,
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        for statement in self.block.get_statements().iter() {
            let statement_generator = StatementGenerator::new(statement, self.local_variables);
            statement_generator.generate()?;
        }

        Ok(())
    }
}
