use std::mem;

use rustyc_ast::{Node, NodeKind, NumberNode, VariableNode};
use rustyc_diagnostics::Diagnostic;
use rustyc_span::Span;
use rustyc_token::{
    BinaryOperatorToken, DelimiterToken, IdentifierToken, NumberToken, Token, TokenKind,
    TokenKindSet,
};

use crate::token_cursor::TokenCursor;

pub struct Parser {
    cursor: TokenCursor,
    token: Token,
    previous_token: Token,
    expected_tokens: TokenKindSet,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Self {
            cursor: TokenCursor::new(tokens),
            token: Token::new_eof(),
            previous_token: Token::new_eof(),
            expected_tokens: TokenKindSet::new(),
        };

        parser.bump();

        parser
    }

    pub fn parse(&mut self) -> rustyc_diagnostics::Result<Vec<Box<Node>>> {
        let mut statements: Vec<Box<Node>> = Vec::new();

        while !self.is_eof() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(statements)
    }

    fn bump(&mut self) {
        self.previous_token = mem::replace(&mut self.token, self.cursor.next());
        self.expected_tokens.clear();
    }

    fn parse_statement(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        self.parse_expression_statement()
    }

    fn parse_expression_statement(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        let low = self.token.get_span().clone();

        let left = self.parse_expression()?;
        self.expect_semicolon()?;

        Ok(self.new_unary_node(NodeKind::ExpressionStatement, &low, left))
    }

    fn parse_expression(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        let low = self.token.get_span().clone();

        let mut node = self.parse_equality()?;

        if self.eat_equal() {
            let right = self.parse_assignment()?;
            node = self.new_binary_node(NodeKind::Assignment, &low, node, right);
        }

        Ok(node)
    }

    fn parse_equality(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        let low = self.token.get_span().clone();

        let mut node = self.parse_relational()?;

        loop {
            if self.eat_equal_equal() {
                let right = self.parse_relational()?;
                node = self.new_binary_node(NodeKind::Equality, &low, node, right);
                continue;
            }

            if self.eat_not_equal() {
                let right = self.parse_relational()?;
                node = self.new_binary_node(NodeKind::NotEqual, &low, node, right);
                continue;
            }

            break;
        }

        Ok(node)
    }

    fn parse_relational(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        let low = self.token.get_span().clone();

        let mut node = self.parse_addition()?;

        loop {
            if self.eat_less_than() {
                let right = self.parse_addition()?;
                node = self.new_binary_node(NodeKind::LessThan, &low, node, right);
                continue;
            }

            if self.eat_less_equal() {
                let right = self.parse_addition()?;
                node = self.new_binary_node(NodeKind::LessThanOrEqual, &low, node, right);
                continue;
            }

            if self.eat_greater_than() {
                let left = self.parse_addition()?;
                node = self.new_binary_node(NodeKind::LessThan, &low, left, node);
                continue;
            }

            if self.eat_greater_equal() {
                let left = self.parse_addition()?;
                node = self.new_binary_node(NodeKind::LessThanOrEqual, &low, left, node);
                continue;
            }

            break;
        }

        Ok(node)
    }

    fn parse_addition(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        let low = self.token.get_span().clone();

        let mut node = self.parse_multiplication()?;

        loop {
            if self.eat_plus() {
                let right = self.parse_multiplication()?;
                node = self.new_binary_node(NodeKind::Addition, &low, node, right);
                continue;
            }

            if self.eat_minus() {
                let right = self.parse_multiplication()?;
                node = self.new_binary_node(NodeKind::Subtraction, &low, node, right);
                continue;
            }

            break;
        }

        Ok(node)
    }

    fn parse_multiplication(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        let low = self.token.get_span().clone();

        let mut node = self.parse_unary()?;

        loop {
            if self.eat_star() {
                let right = self.parse_unary()?;
                node = self.new_binary_node(NodeKind::Multiplication, &low, node, right);
                continue;
            }

            if self.eat_slash() {
                let right = self.parse_unary()?;
                node = self.new_binary_node(NodeKind::Division, &low, node, right);
                continue;
            }

            break;
        }

        Ok(node)
    }

    fn parse_unary(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        let low = self.token.get_span().clone();

        if self.eat_plus() {
            return self.parse_unary();
        }

        if self.eat_minus() {
            let left = self.parse_unary()?;
            return Ok(self.new_unary_node(NodeKind::Negation, &low, left));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        let low = self.token.get_span().clone();

        if self.eat_open_parenthesis() {
            let node = self.parse_expression()?;
            self.expect_close_parenthesis()?;
            return Ok(node);
        }

        if let Some(identifier) = self.eat_identifier() {
            return Ok(self.new_node(
                NodeKind::Variable(VariableNode::new(identifier.get_name())),
                &low,
            ));
        }

        if let Some(number) = self.eat_number() {
            return Ok(self.new_node(NodeKind::Number(NumberNode::new(number.get_value())), &low));
        }

        Err(Diagnostic::new_error(
            rustyc_diagnostics::Error::ExpressionExpected,
            self.token.get_span().clone(),
        ))
    }

    fn new_node(&self, kind: NodeKind, low: &Span) -> Box<Node> {
        Box::new(Node::new(kind, low.to(self.previous_token.get_span())))
    }

    fn new_unary_node(&self, kind: NodeKind, low: &Span, left: Box<Node>) -> Box<Node> {
        Box::new(Node::new_unary(
            kind,
            low.to(self.previous_token.get_span()),
            left,
        ))
    }

    fn new_binary_node(
        &self,
        kind: NodeKind,
        low: &Span,
        left: Box<Node>,
        right: Box<Node>,
    ) -> Box<Node> {
        Box::new(Node::new_binary(
            kind,
            low.to(self.previous_token.get_span()),
            left,
            right,
        ))
    }

    fn expect_close_parenthesis(&mut self) -> rustyc_diagnostics::Result<()> {
        self.expected_tokens.clear();

        if self.eat_close_parenthesis() {
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

    fn unexpected_token(&self) -> Diagnostic {
        Diagnostic::new_error(
            rustyc_diagnostics::Error::UnexpectedToken(
                self.token.get_kind().clone(),
                self.expected_tokens.clone(),
            ),
            self.token.get_span().clone(),
        )
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

    fn eat_semicolon(&mut self) -> bool {
        self.eat(TokenKind::Semicolon)
    }

    fn eat_identifier(&mut self) -> Option<IdentifierToken> {
        let kind = self.token.get_kind().clone();

        if let TokenKind::Identifier(token) = kind {
            self.bump();
            Some(token)
        } else {
            None
        }
    }

    fn eat_number(&mut self) -> Option<NumberToken> {
        let kind = self.token.get_kind().clone();

        if let TokenKind::Number(token) = kind {
            self.bump();
            Some(token)
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

    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn check(&mut self, kind: TokenKind) -> bool {
        let result = (*self.token.get_kind()) == kind;
        self.expected_tokens.insert(kind);

        result
    }

    fn is_eof(&self) -> bool {
        TokenKind::Eof == (*self.token.get_kind())
    }
}
