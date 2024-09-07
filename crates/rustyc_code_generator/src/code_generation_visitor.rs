use rustyc_diagnostics::Diagnostic;
use rustyc_hir::{
    expressions::{
        AssignmentExpression, BinaryExpression, BinaryOperator, Expression, ExpressionKind,
        FunctionCallExpression, UnaryExpression, UnaryOperator, VariableReferenceExpression,
    },
    items::{FunctionItem, Item},
    statements::{IfStatement, LoopStatement, ReturnStatement},
};
use rustyc_hir_visitor::{HirVisitor, HirWalker};

use crate::{
    code_generation_context::CodeGenerationContext, expression_generator::ExpressionGenerator,
    function_generator::FunctionGenerator, item_generator::ItemGenerator,
    statement_generator::StatementGenerator,
};

pub struct CodeGenerationVisitor {
    context: CodeGenerationContext,
    item_generator: ItemGenerator,
    function_generator: FunctionGenerator,
    statement_generator: StatementGenerator,
    expression_generator: ExpressionGenerator,
}

impl CodeGenerationVisitor {
    pub fn new() -> Self {
        Self {
            context: CodeGenerationContext::new(),
            item_generator: ItemGenerator::new(),
            function_generator: FunctionGenerator::new(),
            statement_generator: StatementGenerator::new(),
            expression_generator: ExpressionGenerator::new(),
        }
    }

    fn generate_negate_expression(
        &mut self,
        operand: &Expression,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        self.visit_expression(operand, walker)?;
        self.expression_generator.generate_negate_expression_post();

        Ok(())
    }

    fn generate_address_of_expression(
        &mut self,
        operand: &Expression,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        match operand.get_kind() {
            ExpressionKind::Variable(expression) => self
                .expression_generator
                .generate_address_of_variable(expression, &self.context),
            ExpressionKind::Unary(expression)
                if UnaryOperator::Dereference == *expression.get_operator() =>
            {
                self.visit_expression(&expression.get_operand(), walker)?
            }
            _ => {
                return Err(Diagnostic::new_error(
                    rustyc_diagnostics::Error::NotAnLvalue,
                    operand.get_span().clone(),
                ))
            }
        }

        Ok(())
    }

    fn generate_dereference_expression(
        &mut self,
        operand: &Expression,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        self.visit_expression(operand, walker)?;
        self.expression_generator
            .generate_dereference_expression_post();

        Ok(())
    }
}

impl HirVisitor for CodeGenerationVisitor {
    fn visit_item(&mut self, item: &Item, walker: &HirWalker) -> rustyc_diagnostics::Result<()> {
        self.item_generator.generate_item_separator();
        walker.walk_item(item, self)?;

        Ok(())
    }

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
            .generate_if_statement_else_post(&end_label);

        Ok(())
    }

    fn visit_loop_statement(
        &mut self,
        statement: &LoopStatement,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        let begin_label = self.context.get_label_allocator().allocate_unique("begin");
        let end_label = self.context.get_label_allocator().allocate_unique("end");

        if let Some(initialization_statement) = statement.get_initialization_statement() {
            self.visit_statement(&initialization_statement, walker)?;
        }
        self.statement_generator
            .generate_loop_statement_initialization_post(&begin_label);

        if let Some(condition_expression) = statement.get_condition_expression() {
            self.visit_expression(&condition_expression, walker)?;
            self.statement_generator
                .generate_loop_statement_condition_post(&end_label);
        }

        self.visit_statement(&statement.get_then_statement(), walker)?;

        if let Some(incrementation_expression) = statement.get_incrementation_expression() {
            self.visit_expression(&incrementation_expression, walker)?;
        }
        self.statement_generator
            .generate_loop_statement_incrementation_post(&begin_label, &end_label);

        Ok(())
    }

    fn visit_assignment_expression(
        &mut self,
        expression: &AssignmentExpression,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        self.generate_address_of_expression(&expression.get_left(), walker)?;
        self.expression_generator
            .generate_assignment_expression_left_post();

        self.visit_expression(&expression.get_right(), walker)?;
        self.expression_generator
            .generate_assignment_expression_right_post();

        Ok(())
    }

    fn visit_binary_expression(
        &mut self,
        expression: &BinaryExpression,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        self.visit_expression(&expression.get_right(), walker)?;
        self.expression_generator
            .generate_binary_expression_right_post();

        self.visit_expression(&expression.get_left(), walker)?;
        self.expression_generator
            .generate_binary_expression_left_post();

        match expression.get_operator() {
            BinaryOperator::Equal => self.expression_generator.generate_equal_expression_post(),
            BinaryOperator::NotEqual => self
                .expression_generator
                .generate_not_equal_expression_post(),
            BinaryOperator::LessThan => self
                .expression_generator
                .generate_less_than_expression_post(),
            BinaryOperator::LessThanOrEqual => self
                .expression_generator
                .generate_less_than_or_equal_expression_post(),
            BinaryOperator::Add => self.expression_generator.generate_add_expression_post(),
            BinaryOperator::Subtract => self
                .expression_generator
                .generate_subtract_expression_post(),
            BinaryOperator::Multiply => self
                .expression_generator
                .generate_multiply_expression_post(),
            BinaryOperator::Divide => self.expression_generator.generate_divide_expression_post(),
        }

        Ok(())
    }

    fn visit_unary_expression(
        &mut self,
        expression: &UnaryExpression,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        match expression.get_operator() {
            UnaryOperator::Negate => {
                self.generate_negate_expression(&expression.get_operand(), walker)
            }
            UnaryOperator::AddressOf => {
                self.generate_address_of_expression(&expression.get_operand(), walker)
            }
            UnaryOperator::Dereference => {
                self.generate_dereference_expression(&expression.get_operand(), walker)
            }
        }
    }

    fn visit_variable_expression(
        &mut self,
        expression: &VariableReferenceExpression,
        _walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        self.expression_generator
            .generate_variable_expression(expression, &self.context);

        Ok(())
    }

    fn visit_number_expression(
        &mut self,
        expression: &rustyc_hir::expressions::NumberExpression,
        _walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        self.expression_generator
            .generate_number_expression(expression);

        Ok(())
    }

    fn visit_function_call_expression(
        &mut self,
        expression: &FunctionCallExpression,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        for argument in expression.get_arguments().iter() {
            self.visit_expression(argument, walker)?;
            self.expression_generator
                .generate_function_call_expression_argument_push();
        }

        for argument_index in (0..expression.get_arguments().len()).rev() {
            self.expression_generator
                .generate_function_call_expression_argument_move_to_register(argument_index);
        }

        self.expression_generator
            .generate_function_call_expression_call(expression);

        Ok(())
    }
}
