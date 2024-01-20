use std::mem;

use rustyc_ast::{Node, NodeKind, NumberNode};
use rustyc_diagnostics::Diagnostic;
use rustyc_span::Span;
use rustyc_token::{
    BinaryOperatorToken, DelimiterToken, NumberToken, Token, TokenCategory, TokenKind, TokenKindSet,
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

    pub fn parse(&mut self) -> rustyc_diagnostics::Result<Box<Node>> {
        let expression = self.parse_expression()?;
        self.expect_eof()?;

        Ok(expression)
    }

    fn bump(&mut self) {
        self.previous_token = mem::replace(&mut self.token, self.cursor.next());
        self.expected_tokens.clear();
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

        if self.eat_open_parenthesis() {
            let node = self.parse_expression()?;
            self.expect_close_parenthesis()?;
            return Ok(node);
        }

        if let Some(number) = self.eat_number() {
            return Ok(self.new_node(NodeKind::Number(NumberNode::new(number.get_value())), &low));
        }

        Err(self.token_not_of_category(TokenCategory::Primary))
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

    fn expect_close_parenthesis(&mut self) -> rustyc_diagnostics::Result<()> {
        if self.eat_close_parenthesis() {
            Ok(())
        } else {
            Err(self.unexpected_token())
        }
    }

    fn expect_eof(&mut self) -> rustyc_diagnostics::Result<()> {
        if self.check_eof() {
            Ok(())
        } else {
            Err(Diagnostic::new_error(
                rustyc_diagnostics::Error::EofExpected,
                self.token.get_span().clone(),
            ))
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

    fn token_not_of_category(&self, category: TokenCategory) -> Diagnostic {
        Diagnostic::new_error(
            rustyc_diagnostics::Error::TokenNotOfCategory(self.token.get_kind().clone(), category),
            self.token.get_span().clone(),
        )
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

    fn eat_open_parenthesis(&mut self) -> bool {
        if self.check_open_parenthesis() {
            self.bump();
            true
        } else {
            false
        }
    }

    fn eat_close_parenthesis(&mut self) -> bool {
        if self.check_close_parenthesis() {
            self.bump();
            true
        } else {
            false
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

    fn check_plus(&mut self) -> bool {
        self.check_binary_operator(BinaryOperatorToken::Plus)
    }

    fn check_minus(&mut self) -> bool {
        self.check_binary_operator(BinaryOperatorToken::Minus)
    }

    fn check_star(&mut self) -> bool {
        self.check_binary_operator(BinaryOperatorToken::Star)
    }

    fn check_slash(&mut self) -> bool {
        self.check_binary_operator(BinaryOperatorToken::Slash)
    }

    fn check_open_parenthesis(&mut self) -> bool {
        self.check_open_delimiter(DelimiterToken::Parenthesis)
    }

    fn check_close_parenthesis(&mut self) -> bool {
        self.check_close_delimiter(DelimiterToken::Parenthesis)
    }

    fn check_binary_operator(&mut self, token: BinaryOperatorToken) -> bool {
        self.check(TokenKind::BinaryOperator(token))
    }

    fn check_open_delimiter(&mut self, token: DelimiterToken) -> bool {
        self.check(TokenKind::OpenDelimiter(token))
    }

    fn check_close_delimiter(&mut self, token: DelimiterToken) -> bool {
        self.check(TokenKind::CloseDelimiter(token))
    }

    fn check_eof(&mut self) -> bool {
        self.check(TokenKind::Eof)
    }

    fn check(&mut self, kind: TokenKind) -> bool {
        let result = (*self.token.get_kind()) == kind;
        self.expected_tokens.insert(kind);

        result
    }
}
