use std::mem;

use crate::{
    ast::{Node, NodeKind, NumberNode},
    diagnostics::{self, Diagnostic},
    token::{Token, TokenKind},
};

use super::token_cursor::TokenCursor;

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

    pub fn parse(&mut self) -> diagnostics::Result<Box<Node>> {
        self.parse_expression()
    }

    fn bump(&mut self) {
        self.previous_token = mem::replace(&mut self.token, self.cursor.next());
    }

    fn parse_expression(&mut self) -> diagnostics::Result<Box<Node>> {
        let low = self.token.get_span().clone();

        let mut node = self.parse_multiplication()?;

        Ok(node)
    }

    fn parse_multiplication(&mut self) -> diagnostics::Result<Box<Node>> {
        let low = self.token.get_span().clone();

        let mut node = self.parse_primary()?;

        Ok(node)
    }

    fn parse_primary(&mut self) -> diagnostics::Result<Box<Node>> {
        let low = self.token.get_span().clone();

        let kind = match self.token.get_kind() {
            TokenKind::Number(number) => NodeKind::Number(NumberNode::new(number.get_value())),
            _ => {
                return Err(Diagnostic::new_error(
                    diagnostics::Error::UnexpectedToken,
                    self.token.get_span().clone(),
                ))
            }
        };

        self.bump();

        Ok(Box::new(Node::new(
            kind,
            low.to(self.previous_token.get_span()),
        )))
    }
}
