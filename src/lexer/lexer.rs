use super::{raw_token_cursor::RawTokenCursor, raw_token_kind::RawTokenKind};
use crate::{
    span::Span,
    token::{BinaryOperationToken, Token, TokenKind},
};

pub struct Lexer<'a> {
    source: &'a str,
    cursor: RawTokenCursor<'a>,
    start_position: usize,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            cursor: RawTokenCursor::new(source),
            start_position: 0,
            position: 0,
        }
    }

    pub fn lex(&mut self) -> error::Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            let raw_token = self.cursor.next();

            let start = self.position;
            self.position += raw_token.get_length();

            let kind = match raw_token.get_kind() {
                RawTokenKind::Plus => TokenKind::BinaryOperation(BinaryOperationToken::Plus),
                RawTokenKind::Minus => TokenKind::BinaryOperation(BinaryOperationToken::Minus),
                RawTokenKind::Number => self.lex_number(start),
                RawTokenKind::Whitespace => continue,
                RawTokenKind::Eof => break,
                RawTokenKind::Unknown => todo!(),
            };
        }

        Ok(tokens)
    }

    fn lex_number(&self, start: usize) -> TokenKind {
        let source = self.source_from(start);
        let value = source.parse().map_err(|e| {
            let error = diagnostic::Error::ParseNumber(e);
            self.diagnostic_emitter.emit(Diagnostic::new_error(
                error,
                Span::new(start, self.position),
            ));
            error
        });
    }

    fn source_index(&self, position: usize) -> usize {
        position - self.start_position
    }

    fn source_from(&self, start: usize) -> &'a str {
        self.source_from_to(start, self.position)
    }

    fn source_from_to(&self, start: usize, end: usize) -> &'a str {
        &self.source[self.source_index(start)..self.source_index(end)]
    }

    fn source_from_to_end(&self, start: usize) -> &'a str {
        &self.source[self.source_index(start)..]
    }
}
