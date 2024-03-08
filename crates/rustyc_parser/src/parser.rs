use std::mem;

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

    pub fn parse(mut self) -> rustyc_diagnostics::Result<Item> {
        let low = self.token.get_span().clone();

        let function = self.parse_function()?;

        Ok(Item::new(
            ItemKind::Function(function),
            self.compute_span(&low),
        ))
    }

    fn parse_function(&mut self) -> rustyc_diagnostics::Result<FunctionItem> {
        let body = self.parse_block()?;
        let function = FunctionItem::new(body, self.local_variables.clone());
        self.local_variables.clear();

        Ok(function)
    }

    fn parse_block(&mut self) -> rustyc_diagnostics::Result<Block> {
        let low = self.token.get_span().clone();

        self.expect_open_brace()?;

        let mut statements: Vec<Statement> = Vec::new();

        while !self.eat_close_brace() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(Block::new(statements, self.compute_span(&low)))
    }

    fn parse_statement(&mut self) -> rustyc_diagnostics::Result<Statement> {
        if self.check_keyword(Keyword::Return) {
            return self.parse_return_statement();
        }

        if self.check_open_brace() {
            return self.parse_compound_statement();
        }

        self.parse_expression_statement()
    }

    fn parse_compound_statement(&mut self) -> rustyc_diagnostics::Result<Statement> {
        let low = self.token.get_span().clone();
        let block = self.parse_block()?;

        Ok(Statement::new(
            StatementKind::Compound(block),
            self.compute_span(&low),
        ))
    }

    fn parse_return_statement(&mut self) -> rustyc_diagnostics::Result<Statement> {
        let low = self.token.get_span().clone();

        self.expect_keyword(Keyword::Return)?;

        let expression = self.parse_expression()?;
        self.expect_semicolon()?;

        Ok(Statement::new(
            StatementKind::Return(expression),
            self.compute_span(&low),
        ))
    }

    fn parse_expression_statement(&mut self) -> rustyc_diagnostics::Result<Statement> {
        let low = self.token.get_span().clone();

        if self.eat_semicolon() {
            let span = self.compute_span(&low);
            return Ok(Statement::new(
                StatementKind::Compound(Block::new(Vec::new(), span.clone())),
                span,
            ));
        }

        let expression = self.parse_expression()?;
        self.expect_semicolon()?;

        Ok(Statement::new(
            StatementKind::Expression(expression),
            self.compute_span(&low),
        ))
    }

    fn parse_expression(&mut self) -> rustyc_diagnostics::Result<Box<Expression>> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> rustyc_diagnostics::Result<Box<Expression>> {
        let low = self.token.get_span().clone();

        let mut expression = self.parse_equality()?;

        if self.eat_equal() {
            let right = self.parse_assignment()?;
            expression = self.new_assignment_expression(expression, right, &low);
        }

        Ok(expression)
    }

    fn parse_equality(&mut self) -> rustyc_diagnostics::Result<Box<Expression>> {
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

    fn parse_relational(&mut self) -> rustyc_diagnostics::Result<Box<Expression>> {
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

    fn parse_addition(&mut self) -> rustyc_diagnostics::Result<Box<Expression>> {
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

    fn parse_multiplication(&mut self) -> rustyc_diagnostics::Result<Box<Expression>> {
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

    fn parse_unary(&mut self) -> rustyc_diagnostics::Result<Box<Expression>> {
        let low = self.token.get_span().clone();

        if self.eat_plus() {
            return self.parse_unary();
        }

        if self.eat_minus() {
            let right = self.parse_unary()?;
            return Ok(self.new_unary_expression(UnaryOperator::Negate, right, &low));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> rustyc_diagnostics::Result<Box<Expression>> {
        let low = self.token.get_span().clone();

        if self.eat_open_parenthesis() {
            let expression = self.parse_expression()?;
            self.expect_close_parenthesis()?;
            return Ok(expression);
        }

        if let Some(identifier) = self.eat_identifier() {
            if !self.local_variables.contains(&identifier) {
                self.local_variables.push(identifier.clone());
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

    fn new_assignment_expression(
        &self,
        left: Box<Expression>,
        right: Box<Expression>,
        low: &Span,
    ) -> Box<Expression> {
        self.new_expression(ExpressionKind::Assignment(left, right), low)
    }

    fn new_binary_expression(
        &self,
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
        low: &Span,
    ) -> Box<Expression> {
        self.new_expression(ExpressionKind::Binary(operator, left, right), low)
    }

    fn new_unary_expression(
        &self,
        operator: UnaryOperator,
        right: Box<Expression>,
        low: &Span,
    ) -> Box<Expression> {
        self.new_expression(ExpressionKind::Unary(operator, right), low)
    }

    fn new_variable_expression(&self, name: String, low: &Span) -> Box<Expression> {
        self.new_expression(ExpressionKind::Variable(name), low)
    }

    fn new_number_expression(&self, value: u64, low: &Span) -> Box<Expression> {
        self.new_expression(ExpressionKind::Number(value), low)
    }

    fn new_expression(&self, kind: ExpressionKind, low: &Span) -> Box<Expression> {
        Box::new(Expression::new(kind, self.compute_span(low)))
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

    fn bump(&mut self) {
        self.previous_token = mem::replace(&mut self.token, self.cursor.next());
        self.expected_tokens.clear();
    }

    fn check_open_brace(&mut self) -> bool {
        self.check_open_delimiter(DelimiterToken::Brace)
    }

    fn check_open_delimiter(&mut self, token: DelimiterToken) -> bool {
        self.check(TokenKind::OpenDelimiter(token))
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

    fn compute_span(&self, low: &Span) -> Span {
        low.to(self.previous_token.get_span())
    }
}
