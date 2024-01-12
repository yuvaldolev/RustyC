use crate::{ast::Node, token::Token};

use super::token_cursor::TokenCursor;

pub struct Parser {
    cursor: TokenCursor,
    token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Self {
            cursor: TokenCursor::new(tokens),
            token: Token::new_eof(),
        };

        parser.bump();

        parser
    }

    pub fn parse(&mut self) -> Box<Node> {}

    fn bump(&mut self) {
        self.token = self.cursor.next();
    }
}
