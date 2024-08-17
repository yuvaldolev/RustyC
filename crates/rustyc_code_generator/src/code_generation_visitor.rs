use rustyc_hir::{
    items::FunctionItem,
    statements::{IfStatement, ReturnStatement},
};
use rustyc_hir_walker::{HirVisitor, HirWalker};

use crate::{
    code_generation_context::CodeGenerationContext, function_generator::FunctionGenerator,
    statement_generator::StatementGenerator,
};

pub struct CodeGenerationVisitor {
    context: CodeGenerationContext,
    function_generator: FunctionGenerator,
    statement_generator: StatementGenerator,
}

impl CodeGenerationVisitor {
    pub fn new() -> Self {
        Self {
            context: CodeGenerationContext::new(),
            function_generator: FunctionGenerator::new(),
            statement_generator: StatementGenerator::new(),
        }
    }
}

impl HirVisitor for CodeGenerationVisitor {
    fn visit_function(
        &mut self,
        function: &FunctionItem,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        self.context.enter_function(&function);

        self.function_generator
            .generate_prologue(function, &self.context);
        self.function_generator
            .generate_push_parameters_to_stack(function, &self.context);

        walker.walk_function(function, self)?;

        self.function_generator.generate_epilogue(&mut self.context);

        self.context.exit_function();

        Ok(())
    }

    fn visit_return_statement(
        &mut self,
        statement: &ReturnStatement,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        walker.walk_return_statement(statement, self)?;

        self.statement_generator
            .generate_return_statement_post(&mut self.context);

        Ok(())
    }

    fn visit_if_statement(
        &mut self,
        statement: &IfStatement,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        let else_label = self.context.get_label_allocator().allocate_unique("else");
        let end_label = self.context.get_label_allocator().allocate_unique("end");

        self.visit_expression(&statement.get_condition_expression(), walker)?;
        self.statement_generator
            .generate_if_statement_condition_post(&else_label);

        self.visit_statement(&statement.get_then_statement(), walker)?;
        self.statement_generator
            .generate_if_statement_then_post(&end_label);

        self.statement_generator
            .generate_if_statement_else_pre(&else_label);
        if let Some(else_statement) = statement.get_else_statement() {
            self.visit_statement(&else_statement, walker)?;
        }

        self.statement_generator
            .generate_if_statement_end(&end_label);

        Ok(())
    }
}
