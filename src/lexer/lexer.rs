use std::mem;

use crate::{
    diagnostics::{self, Diagnostic},
    span::Span,
    token::{Base, BinaryOperatorToken, Token, TokenKind},
};

use super::{raw_token_cursor::RawTokenCursor, raw_token_kind::RawTokenKind};

pub struct Lexer<'a> {
    source: &'a str,
    cursor: RawTokenCursor<'a>,
    token: Token,
    start_position: usize,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> diagnostics::Result<Self> {
        let mut lexer = Self {
            source,
            cursor: RawTokenCursor::new(source),
            token: Token::new_eof(),
            start_position: 0,
            position: 0,
        };

        lexer.bump()?;

        Ok(lexer)
    }

    pub fn lex(&mut self) -> diagnostics::Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            match self.token.get_kind() {
                TokenKind::Eof => break,
                _ => tokens.push(self.bump()?),
            }
        }

        Ok(tokens)
    }

    fn bump(&mut self) -> diagnostics::Result<Token> {
        let next_token = self.lex_token()?;

        // TODO: Perform token gluing here.

        let this_token = mem::replace(&mut self.token, next_token);

        Ok(this_token)
    }

    fn lex_token(&mut self) -> diagnostics::Result<Token> {
        loop {
            let raw_token = self.cursor.next();

            let start = self.position;
            self.position += raw_token.get_length();

            let kind = match raw_token.get_kind() {
                RawTokenKind::Plus => TokenKind::BinaryOperator(BinaryOperatorToken::Plus),
                RawTokenKind::Minus => TokenKind::BinaryOperator(BinaryOperatorToken::Minus),
                RawTokenKind::Multiply => TokenKind::BinaryOperator(BinaryOperatorToken::Multiply),
                RawTokenKind::Divide => TokenKind::BinaryOperator(BinaryOperatorToken::Divide),
                RawTokenKind::Number => self.lex_number(start)?,
                RawTokenKind::Whitespace => continue,
                RawTokenKind::Eof => TokenKind::Eof,
                RawTokenKind::Unknown => {
                    return Err(Diagnostic::new_error(
                        diagnostics::Error::UnknownTokenStart,
                        self.span_from(start),
                    ))
                }
            };

            return Ok(Token::new(kind, self.span_from(start)));
        }
    }

    fn lex_number(&mut self, start: usize) -> diagnostics::Result<TokenKind> {
        let source = self.source_from(start);
        let value = source.parse().map_err(|e| {
            Diagnostic::new_error(diagnostics::Error::ParseNumber(e), self.span_from(start))
        })?;

        Ok(TokenKind::Number(Base::Decimal, value))
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

    fn span_from(&self, start: usize) -> Span {
        Span::new(start, self.position)
    }
}
