use std::str::Chars;

use crate::{error, token::Token};

pub struct Lexer {
    source: String,
}

const EOF_CHAR: char = '\0';

impl Lexer {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn lex(&self) -> error::Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut chars = self.source.chars();

        while let Some(token) = self.lex_token(&mut chars)? {
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn lex_token(&self, chars: &mut Chars) -> error::Result<Option<Token>> {
        Self::eat_whitespace(chars);

        if Self::is_eof(chars) {
            return Ok(None);
        }

        let at = Self::peek_first(chars);
        let start_index = self.index(chars);
        Self::advance(chars);

        if at.is_ascii_digit() {
            return self.lex_number(chars, start_index).map(Some);
        }

        match at {
            '-' | '+' => Ok(Some(Token::Punctuator(at))),
            _ => Err(error::Error::UnexpectedCharacter(at)),
        }
    }

    fn is_eof(chars: &Chars) -> bool {
        chars.clone().next().is_none()
    }

    fn peek_first(chars: &Chars) -> char {
        Self::peek_nth(chars, 0)
    }

    fn peek_nth(chars: &Chars, n: usize) -> char {
        chars.clone().nth(n).unwrap_or(EOF_CHAR)
    }

    fn advance(chars: &mut Chars) {
        chars.next();
    }

    fn eat_whitespace(chars: &mut Chars) {
        while Self::peek_first(chars).is_whitespace() {
            Self::advance(chars)
        }
    }

    fn eat_digits(chars: &mut Chars) {
        while Self::peek_first(chars).is_ascii_digit() {
            Self::advance(chars);
        }
    }

    fn lex_number(&self, chars: &mut Chars, start_index: usize) -> error::Result<Token> {
        Self::eat_digits(chars);

        let source = self.slice_source(start_index, self.index(chars));
        let value = source
            .parse()
            .map_err(|e| error::Error::ParseNumber(e, source.to_owned()))?;

        Ok(Token::Number(value))
    }

    fn index(&self, chars: &Chars) -> usize {
        self.source.len() - chars.as_str().len()
    }

    fn slice_source(&self, start: usize, end: usize) -> &str {
        &self.source[start..end]
    }
}
