use crate::token::Token;

pub struct TokenCursor {
    tokens: Vec<Token>,
    index: usize,
}

impl TokenCursor {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn next(&mut self) -> Token {
        self.tokens
            .get(self.index)
            .map_or(Token::new_eof(), |token| {
                self.index += 1;
                token.clone()
            })
    }
}
