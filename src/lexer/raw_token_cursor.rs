use std::str::Chars;

use super::{raw_token::RawToken, raw_token_kind::RawTokenKind};

pub struct RawTokenCursor<'a> {
    chars: Chars<'a>,
    length_remaining: usize,
}

const EOF_CHARACTER: char = '\0';

impl<'a> RawTokenCursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars(),
            length_remaining: source.len(),
        }
    }

    pub fn next(&mut self) -> RawToken {
        let Some(first_character) = self.bump() else {
            return RawToken::new(RawTokenKind::Eof, 0);
        };

        let kind = match first_character {
            c if c.is_whitespace() => {
                self.eat_whitespace();
                RawTokenKind::Whitespace
            }
            c if c.is_ascii_digit() => {
                self.eat_number();
                RawTokenKind::Number
            }
            '+' => RawTokenKind::Plus,
            '-' => RawTokenKind::Minus,
            '*' => RawTokenKind::Star,
            '/' => RawTokenKind::Slash,
            '(' => RawTokenKind::OpenParenthesis,
            ')' => RawTokenKind::CloseParenthesis,
            _ => RawTokenKind::Unknown,
        };

        let token = RawToken::new(kind, self.position_within_token());
        self.reset_position_within_token();

        token
    }

    fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHARACTER)
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn position_within_token(&self) -> usize {
        self.length_remaining - self.chars.as_str().len()
    }

    fn reset_position_within_token(&mut self) {
        self.length_remaining = self.chars.as_str().len()
    }

    fn eat_whitespace(&mut self) {
        self.eat_while(char::is_whitespace);
    }

    fn eat_number(&mut self) {
        self.eat_while(|c| c.is_ascii_digit());
    }

    fn eat_while(&mut self, predicate: impl Fn(char) -> bool) {
        while predicate(self.first()) && (!self.is_eof()) {
            self.bump();
        }
    }
}
