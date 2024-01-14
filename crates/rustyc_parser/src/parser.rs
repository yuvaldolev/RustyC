use std::mem;

use rustyc_ast::{Node, NodeKind, NumberNode};
use rustyc_diagnostics::Diagnostic;
use rustyc_span::Span;
use rustyc_token::{BinaryOperatorToken, Token, TokenKind};

use crate::token_cursor::TokenCursor;

pub struct Parser {
    cursor: TokenCursor,
    token: Token,
    previous_token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Self {
            cursor: TokenCursor::new(tokens),
            token: Token::new_eof(),
            previous_token: Token::new_eof(),
        };

        parser.bump();

        parser
    }

    pub fn parse(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        self.parse_expression()
    }

    fn bump(&mut self) {
        self.previous_token = mem::replace(&mut self.token, self.cursor.next());
    }

    fn parse_expression(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        let low = self.token.get_span().clone();

        let mut node = self.parse_multiplication()?;

        loop {
            if self.eat_plus() {
                let right = self.parse_multiplication()?;
                node = self.new_binary_node(NodeKind::Add, &low, node, right);
                continue;
            }

            if self.eat_minus() {
                let right = self.parse_multiplication()?;
                node = self.new_binary_node(NodeKind::Subtract, &low, node, right);
                continue;
            }

            break;
        }

        Ok(node)
    }

    fn parse_multiplication(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        let low = self.token.get_span().clone();

        let mut node = self.parse_primary()?;

        loop {
            if self.eat_star() {
                let right = self.parse_primary()?;
                node = self.new_binary_node(NodeKind::Multiply, &low, node, right);
                continue;
            }

            if self.eat_slash() {
                let right = self.parse_primary()?;
                node = self.new_binary_node(NodeKind::Divide, &low, node, right);
                continue;
            }

            break;
        }

        Ok(node)
    }

    fn parse_primary(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        let low = self.token.get_span().clone();

        // if self.eat_open_parenthesis() {}

        let kind = match self.token.get_kind() {
            TokenKind::Number(number) => NodeKind::Number(NumberNode::new(number.get_value())),
            TokenKind::Eof => {
                return Err(Diagnostic::new_error(
                    rustyc_diagnostics::Error::UnexpectedEof,
                    Span::new_dummy(),
                ))
            }
            _ => {
                return Err(Diagnostic::new_error(
                    rustyc_diagnostics::Error::UnexpectedToken,
                    self.token.get_span().clone(),
                ))
            }
        };

        self.bump();

        Ok(self.new_node(kind, &low))
    }

    fn new_node(&self, kind: NodeKind, low: &Span) -> Box<Node> {
        Box::new(Node::new(kind, low.to(self.previous_token.get_span())))
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

    fn eat_plus(&mut self) -> bool {
        if self.check_plus() {
            self.bump();
            true
        } else {
            false
        }
    }

    fn eat_minus(&mut self) -> bool {
        if self.check_minus() {
            self.bump();
            true
        } else {
            false
        }
    }

    fn eat_star(&mut self) -> bool {
        if self.check_star() {
            self.bump();
            true
        } else {
            false
        }
    }

    fn eat_slash(&mut self) -> bool {
        if self.check_slash() {
            self.bump();
            true
        } else {
            false
        }
    }

    fn check_plus(&self) -> bool {
        self.check(TokenKind::BinaryOperator(BinaryOperatorToken::Plus))
    }

    fn check_minus(&self) -> bool {
        self.check(TokenKind::BinaryOperator(BinaryOperatorToken::Minus))
    }

    fn check_star(&self) -> bool {
        self.check(TokenKind::BinaryOperator(BinaryOperatorToken::Star))
    }

    fn check_slash(&self) -> bool {
        self.check(TokenKind::BinaryOperator(BinaryOperatorToken::Slash))
    }

    fn check(&self, kind: TokenKind) -> bool {
        (*self.token.get_kind()) == kind
    }
}
