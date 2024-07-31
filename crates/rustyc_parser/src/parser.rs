use std::{mem, rc::Rc};

use rustyc_ast::{
    BinaryOperator, Block, Expression, ExpressionKind, FunctionItem, Item, ItemKind, Statement,
    StatementKind, UnaryOperator,
};
use rustyc_diagnostics::Diagnostic;
use rustyc_span::Span;
use rustyc_token::{BinaryOperatorToken, DelimiterToken, Keyword, Token, TokenKind, TokenKindSet};

use crate::token_cursor::TokenCursor;

pub struct Parser {
    cursor: TokenCursor,
    token: Token,
    previous_token: Token,
    expected_tokens: TokenKindSet,
    local_variables: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Self {
            cursor: TokenCursor::new(tokens),
            token: Token::new_eof(),
            previous_token: Token::new_eof(),
            expected_tokens: TokenKindSet::new(),
            local_variables: Vec::new(),
        };

        parser.bump();

        parser
    }

    pub fn parse(mut self) -> rustyc_diagnostics::Result<Vec<Rc<Item>>> {
        let mut ast: Vec<Rc<Item>> = Vec::new();

        while !self.is_eof() {
            ast.push(self.parse_item()?);
        }

        Ok(ast)
    }

    fn parse_item(&mut self) -> rustyc_diagnostics::Result<Rc<Item>> {
        let low = self.token.get_span().clone();

        let function = self.parse_function()?;

        Ok(Rc::new(Item::new(
            ItemKind::Function(function),
            self.compute_span(&low),
        )))
    }

    fn parse_function(&mut self) -> rustyc_diagnostics::Result<Rc<FunctionItem>> {
        let name = self.expect_identifier()?;

        self.expect_open_parenthesis()?;

        let parameters = if self.check_close_parenthesis() {
            Vec::new()
        } else {
            self.parse_function_parameters()?
        };

        self.expect_close_parenthesis()?;

        let body = self.parse_block()?;

        let function = Rc::new(FunctionItem::new(
            name,
            parameters,
            body,
            self.local_variables.clone(),
        ));

        self.local_variables.clear();

        Ok(function)
    }

    fn parse_block(&mut self) -> rustyc_diagnostics::Result<Rc<Block>> {
        let low = self.token.get_span().clone();

        self.expect_open_brace()?;

        let mut statements: Vec<Rc<Statement>> = Vec::new();

        while !self.eat_close_brace() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(Rc::new(Block::new(statements, self.compute_span(&low))))
    }

    fn parse_statement(&mut self) -> rustyc_diagnostics::Result<Rc<Statement>> {
        let low = self.token.get_span().clone();

        let kind = if self.check_keyword(Keyword::Return) {
            self.parse_return_statement()?
        } else if self.check_keyword(Keyword::If) {
            self.parse_if_statement()?
        } else if self.check_keyword(Keyword::For) {
            self.parse_for_statement()?
        } else if self.check_keyword(Keyword::While) {
            self.parse_while_statement()?
        } else if self.check_open_brace() {
            self.parse_compound_statement()?
        } else {
            self.parse_expression_statement()?
        };

        Ok(Rc::new(Statement::new(kind, self.compute_span(&low))))
    }

    fn parse_return_statement(&mut self) -> rustyc_diagnostics::Result<StatementKind> {
        self.expect_keyword(Keyword::Return)?;

        let expression = self.parse_expression()?;
        self.expect_semicolon()?;

        Ok(StatementKind::Return(expression))
    }

    fn parse_if_statement(&mut self) -> rustyc_diagnostics::Result<StatementKind> {
        self.expect_keyword(Keyword::If)?;

        self.expect_open_parenthesis()?;
        let condition_expression = self.parse_expression()?;
        self.expect_close_parenthesis()?;

        let then_statement = self.parse_statement()?;

        let else_statement = if self.eat_keyword(Keyword::Else) {
            Some(self.parse_statement()?)
        } else {
            None
        };

        Ok(StatementKind::If(
            condition_expression,
            then_statement,
            else_statement,
        ))
    }

    fn parse_for_statement(&mut self) -> rustyc_diagnostics::Result<StatementKind> {
        self.expect_keyword(Keyword::For)?;

        self.expect_open_parenthesis()?;

        let low = self.token.get_span().clone();
        let initialization_statement = Rc::new(Statement::new(
            self.parse_expression_statement()?,
            self.compute_span(&low),
        ));

        let condition_expression = if self.check_semicolon() {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.expect_semicolon()?;

        let incrementation_expression = if self.check_close_parenthesis() {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.expect_close_parenthesis()?;

        let then_statement = self.parse_statement()?;

        Ok(StatementKind::Loop(
            Some(initialization_statement),
            condition_expression,
            incrementation_expression,
            then_statement,
        ))
    }

    fn parse_while_statement(&mut self) -> rustyc_diagnostics::Result<StatementKind> {
        self.expect_keyword(Keyword::While)?;

        self.expect_open_parenthesis()?;
        let condition_expression = self.parse_expression()?;
        self.expect_close_parenthesis()?;

        let then_statement = self.parse_statement()?;

        Ok(StatementKind::Loop(
            None,
            Some(condition_expression),
            None,
            then_statement,
        ))
    }

    fn parse_compound_statement(&mut self) -> rustyc_diagnostics::Result<StatementKind> {
        let block = self.parse_block()?;
        Ok(StatementKind::Compound(block))
    }

    fn parse_expression_statement(&mut self) -> rustyc_diagnostics::Result<StatementKind> {
        let low = self.token.get_span().clone();

        if self.eat_semicolon() {
            return Ok(StatementKind::Compound(Rc::new(Block::new(
                Vec::new(),
                self.compute_span(&low),
            ))));
        }

        let expression = self.parse_expression()?;
        self.expect_semicolon()?;

        Ok(StatementKind::Expression(expression))
    }

    fn parse_expression(&mut self) -> rustyc_diagnostics::Result<Rc<Expression>> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> rustyc_diagnostics::Result<Rc<Expression>> {
        let low = self.token.get_span().clone();

        let mut expression = self.parse_equality()?;

        if self.eat_equal() {
            let right = self.parse_assignment()?;
            expression = self.new_assignment_expression(expression, right, &low);
        }

        Ok(expression)
    }

    fn parse_equality(&mut self) -> rustyc_diagnostics::Result<Rc<Expression>> {
        let low = self.token.get_span().clone();

        let mut expression = self.parse_relational()?;

        loop {
            if self.eat_equal_equal() {
                let right = self.parse_relational()?;
                expression =
                    self.new_binary_expression(BinaryOperator::Equal, expression, right, &low);
                continue;
            }

            if self.eat_not_equal() {
                let right = self.parse_relational()?;
                expression =
                    self.new_binary_expression(BinaryOperator::NotEqual, expression, right, &low);
                continue;
            }

            break;
        }

        Ok(expression)
    }

    fn parse_relational(&mut self) -> rustyc_diagnostics::Result<Rc<Expression>> {
        let low = self.token.get_span().clone();

        let mut expression = self.parse_addition()?;

        loop {
            if self.eat_less_than() {
                let right = self.parse_addition()?;
                expression =
                    self.new_binary_expression(BinaryOperator::LessThan, expression, right, &low);
                continue;
            }

            if self.eat_less_equal() {
                let right = self.parse_addition()?;
                expression = self.new_binary_expression(
                    BinaryOperator::LessThanOrEqual,
                    expression,
                    right,
                    &low,
                );
                continue;
            }

            if self.eat_greater_than() {
                let left = self.parse_addition()?;
                expression =
                    self.new_binary_expression(BinaryOperator::LessThan, left, expression, &low);
                continue;
            }

            if self.eat_greater_equal() {
                let left = self.parse_addition()?;
                expression = self.new_binary_expression(
                    BinaryOperator::LessThanOrEqual,
                    left,
                    expression,
                    &low,
                );
                continue;
            }

            break;
        }

        Ok(expression)
    }

    fn parse_addition(&mut self) -> rustyc_diagnostics::Result<Rc<Expression>> {
        let low = self.token.get_span().clone();

        let mut expression = self.parse_multiplication()?;

        loop {
            if self.eat_plus() {
                let right = self.parse_multiplication()?;
                expression =
                    self.new_binary_expression(BinaryOperator::Add, expression, right, &low);
                continue;
            }

            if self.eat_minus() {
                let right = self.parse_multiplication()?;
                expression =
                    self.new_binary_expression(BinaryOperator::Subtract, expression, right, &low);
                continue;
            }

            break;
        }

        Ok(expression)
    }

    fn parse_multiplication(&mut self) -> rustyc_diagnostics::Result<Rc<Expression>> {
        let low = self.token.get_span().clone();

        let mut expression = self.parse_unary()?;

        loop {
            if self.eat_star() {
                let right = self.parse_unary()?;
                expression =
                    self.new_binary_expression(BinaryOperator::Multiply, expression, right, &low);
                continue;
            }

            if self.eat_slash() {
                let right = self.parse_unary()?;
                expression =
                    self.new_binary_expression(BinaryOperator::Divide, expression, right, &low);
                continue;
            }

            break;
        }

        Ok(expression)
    }

    fn parse_unary(&mut self) -> rustyc_diagnostics::Result<Rc<Expression>> {
        let low = self.token.get_span().clone();

        if self.eat_plus() {
            return self.parse_unary();
        }

        if self.eat_minus() {
            let right = self.parse_unary()?;
            return Ok(self.new_unary_expression(UnaryOperator::Negate, right, &low));
        }

        if self.eat_and() {
            let right = self.parse_unary()?;
            return Ok(self.new_unary_expression(UnaryOperator::AddressOf, right, &low));
        }

        if self.eat_star() {
            let right = self.parse_unary()?;
            return Ok(self.new_unary_expression(UnaryOperator::Dereference, right, &low));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> rustyc_diagnostics::Result<Rc<Expression>> {
        let low = self.token.get_span().clone();

        if self.eat_open_parenthesis() {
            let expression = self.parse_expression()?;
            self.expect_close_parenthesis()?;
            return Ok(expression);
        }

        if let Some(identifier) = self.eat_identifier() {
            if self.eat_open_parenthesis() {
                return self.parse_function_call(identifier, &low);
            }

            // TODO: Currently, variable accesses also add a variable to the local
            // variables list. This should only be done for variable declarations.
            if !self.local_variables.contains(&identifier) {
                self.local_variables.insert(0, identifier.clone());
            }

            return Ok(self.new_variable_expression(identifier, &low));
        }

        if let Some(number) = self.eat_number() {
            return Ok(self.new_number_expression(number, &low));
        }

        Err(Diagnostic::new_error(
            rustyc_diagnostics::Error::ExpressionExpected,
            self.token.get_span().clone(),
        ))
    }

    fn parse_function_parameters(&mut self) -> rustyc_diagnostics::Result<Vec<String>> {
        let mut parameters: Vec<String> = Vec::new();

        loop {
            let parameter = self.expect_identifier()?;
            self.local_variables.push(parameter.clone());
            parameters.push(parameter);

            if !self.eat_comma() {
                break;
            }
        }

        Ok(parameters)
    }

    fn parse_function_call(
        &mut self,
        name: String,
        low: &Span,
    ) -> rustyc_diagnostics::Result<Rc<Expression>> {
        let arguments = if self.check_close_parenthesis() {
            Vec::new()
        } else {
            self.parse_function_call_arguments()?
        };

        self.expect_close_parenthesis()?;

        Ok(self.new_function_call_expression(name, arguments, low))
    }

    fn parse_function_call_arguments(&mut self) -> rustyc_diagnostics::Result<Vec<Rc<Expression>>> {
        let mut arguments: Vec<Rc<Expression>> = Vec::new();

        loop {
            arguments.push(self.parse_assignment()?);

            if !self.eat_comma() {
                break;
            }
        }

        Ok(arguments)
    }

    fn new_assignment_expression(
        &self,
        left: Rc<Expression>,
        right: Rc<Expression>,
        low: &Span,
    ) -> Rc<Expression> {
        self.new_expression(ExpressionKind::Assignment(left, right), low)
    }

    fn new_binary_expression(
        &self,
        operator: BinaryOperator,
        left: Rc<Expression>,
        right: Rc<Expression>,
        low: &Span,
    ) -> Rc<Expression> {
        self.new_expression(ExpressionKind::Binary(operator, left, right), low)
    }

    fn new_unary_expression(
        &self,
        operator: UnaryOperator,
        right: Rc<Expression>,
        low: &Span,
    ) -> Rc<Expression> {
        self.new_expression(ExpressionKind::Unary(operator, right), low)
    }

    fn new_variable_expression(&self, name: String, low: &Span) -> Rc<Expression> {
        self.new_expression(ExpressionKind::Variable(name), low)
    }

    fn new_number_expression(&self, value: u64, low: &Span) -> Rc<Expression> {
        self.new_expression(ExpressionKind::Number(value), low)
    }

    fn new_function_call_expression(
        &self,
        name: String,
        arguments: Vec<Rc<Expression>>,
        low: &Span,
    ) -> Rc<Expression> {
        self.new_expression(ExpressionKind::FunctionCall(name, arguments), low)
    }

    fn new_expression(&self, kind: ExpressionKind, low: &Span) -> Rc<Expression> {
        Rc::new(Expression::new(kind, self.compute_span(low)))
    }

    fn expect_open_parenthesis(&mut self) -> rustyc_diagnostics::Result<()> {
        self.expected_tokens.clear();

        if self.eat_open_parenthesis() {
            Ok(())
        } else {
            Err(self.unexpected_token())
        }
    }

    fn expect_close_parenthesis(&mut self) -> rustyc_diagnostics::Result<()> {
        self.expected_tokens.clear();

        if self.eat_close_parenthesis() {
            Ok(())
        } else {
            Err(self.unexpected_token())
        }
    }

    fn expect_open_brace(&mut self) -> rustyc_diagnostics::Result<()> {
        self.expected_tokens.clear();

        if self.eat_open_brace() {
            Ok(())
        } else {
            Err(self.unexpected_token())
        }
    }

    fn expect_semicolon(&mut self) -> rustyc_diagnostics::Result<()> {
        self.expected_tokens.clear();

        if self.eat_semicolon() {
            Ok(())
        } else {
            Err(self.unexpected_token())
        }
    }

    fn expect_keyword(&mut self, keyword: Keyword) -> rustyc_diagnostics::Result<()> {
        self.expected_tokens.clear();

        if self.eat_keyword(keyword) {
            Ok(())
        } else {
            Err(self.unexpected_token())
        }
    }

    fn expect_identifier(&mut self) -> rustyc_diagnostics::Result<String> {
        self.expected_tokens.clear();

        if let Some(identifier) = self.eat_identifier() {
            Ok(identifier)
        } else {
            Err(self.unexpected_token())
        }
    }

    fn unexpected_token(&self) -> Diagnostic {
        let error = if self.expected_tokens.len() > 1 {
            rustyc_diagnostics::Error::UnexpectedTokenMultiple(
                self.token.get_kind().clone(),
                self.expected_tokens.clone(),
            )
        } else {
            rustyc_diagnostics::Error::UnexpectedTokenSingle(
                self.token.get_kind().clone(),
                self.expected_tokens.first().clone(),
            )
        };

        Diagnostic::new_error(error, self.token.get_span().clone())
    }

    fn eat_equal(&mut self) -> bool {
        self.eat(TokenKind::Equal)
    }

    fn eat_equal_equal(&mut self) -> bool {
        self.eat(TokenKind::EqualEqual)
    }

    fn eat_not_equal(&mut self) -> bool {
        self.eat(TokenKind::NotEqual)
    }

    fn eat_less_than(&mut self) -> bool {
        self.eat(TokenKind::LessThan)
    }

    fn eat_less_equal(&mut self) -> bool {
        self.eat(TokenKind::LessEqual)
    }

    fn eat_greater_than(&mut self) -> bool {
        self.eat(TokenKind::GreaterThan)
    }

    fn eat_greater_equal(&mut self) -> bool {
        self.eat(TokenKind::GreaterEqual)
    }

    fn eat_plus(&mut self) -> bool {
        self.eat_binary_operator(BinaryOperatorToken::Plus)
    }

    fn eat_minus(&mut self) -> bool {
        self.eat_binary_operator(BinaryOperatorToken::Minus)
    }

    fn eat_star(&mut self) -> bool {
        self.eat_binary_operator(BinaryOperatorToken::Star)
    }

    fn eat_slash(&mut self) -> bool {
        self.eat_binary_operator(BinaryOperatorToken::Slash)
    }

    fn eat_and(&mut self) -> bool {
        self.eat_binary_operator(BinaryOperatorToken::And)
    }

    fn eat_open_parenthesis(&mut self) -> bool {
        self.eat_open_delimiter(DelimiterToken::Parenthesis)
    }

    fn eat_close_parenthesis(&mut self) -> bool {
        self.eat_close_delimiter(DelimiterToken::Parenthesis)
    }

    fn eat_open_brace(&mut self) -> bool {
        self.eat_open_delimiter(DelimiterToken::Brace)
    }

    fn eat_close_brace(&mut self) -> bool {
        self.eat_close_delimiter(DelimiterToken::Brace)
    }

    fn eat_semicolon(&mut self) -> bool {
        self.eat(TokenKind::Semicolon)
    }

    fn eat_comma(&mut self) -> bool {
        self.eat(TokenKind::Comma)
    }

    fn eat_identifier(&mut self) -> Option<String> {
        let kind = self.token.get_kind().clone();

        if let TokenKind::Identifier(name) = kind {
            self.bump();
            Some(name)
        } else {
            None
        }
    }

    fn eat_number(&mut self) -> Option<u64> {
        let kind = self.token.get_kind().clone();

        if let TokenKind::Number(value) = kind {
            self.bump();
            Some(value)
        } else {
            None
        }
    }

    fn eat_binary_operator(&mut self, token: BinaryOperatorToken) -> bool {
        self.eat(TokenKind::BinaryOperator(token))
    }

    fn eat_open_delimiter(&mut self, token: DelimiterToken) -> bool {
        self.eat(TokenKind::OpenDelimiter(token))
    }

    fn eat_close_delimiter(&mut self, token: DelimiterToken) -> bool {
        self.eat(TokenKind::CloseDelimiter(token))
    }

    fn eat_keyword(&mut self, keyword: Keyword) -> bool {
        if self.check_keyword(keyword) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn check_open_brace(&mut self) -> bool {
        self.check_open_delimiter(DelimiterToken::Brace)
    }

    fn check_close_parenthesis(&mut self) -> bool {
        self.check_close_delimiter(DelimiterToken::Parenthesis)
    }

    fn check_semicolon(&mut self) -> bool {
        self.check(TokenKind::Semicolon)
    }

    fn check_open_delimiter(&mut self, token: DelimiterToken) -> bool {
        self.check(TokenKind::OpenDelimiter(token))
    }

    fn check_close_delimiter(&mut self, token: DelimiterToken) -> bool {
        self.check(TokenKind::CloseDelimiter(token))
    }

    fn check(&mut self, kind: TokenKind) -> bool {
        let result = (*self.token.get_kind()) == kind;
        self.expected_tokens.insert(kind);

        result
    }

    fn check_keyword(&self, keyword: Keyword) -> bool {
        // TODO: Add a `TokenCategory` enum and save expected tokens as instances of
        // this enum. Then add a the keyword to the expected tokens.
        self.token.is_keyword(&keyword)
    }

    fn is_eof(&self) -> bool {
        TokenKind::Eof == *self.token.get_kind()
    }

    fn bump(&mut self) {
        self.previous_token = mem::replace(&mut self.token, self.cursor.next());
        self.expected_tokens.clear();
    }

    fn compute_span(&self, low: &Span) -> Span {
        low.to(self.previous_token.get_span())
    }
}
