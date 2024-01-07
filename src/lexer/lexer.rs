use crate::{
    diagnostics::{self, Diagnostic, DiagnosticEmitter},
    span::Span,
    token::{Base, BinaryOperatorToken, Token, TokenKind},
};

use super::{raw_token_cursor::RawTokenCursor, raw_token_kind::RawTokenKind};

pub struct Lexer<'a> {
    source: &'a str,
    diagnostic_emitter: &'a mut DiagnosticEmitter,
    cursor: RawTokenCursor<'a>,
    start_position: usize,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, diagnostic_emitter: &'a mut DiagnosticEmitter) -> Self {
        Self {
            source,
            diagnostic_emitter,
            cursor: RawTokenCursor::new(source),
            start_position: 0,
            position: 0,
        }
    }

    pub fn lex(&mut self) -> diagnostics::Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            let raw_token = self.cursor.next();

            let start = self.position;
            self.position += raw_token.get_length();

            let kind = match raw_token.get_kind() {
                RawTokenKind::Plus => TokenKind::BinaryOperator(BinaryOperatorToken::Plus),
                RawTokenKind::Minus => TokenKind::BinaryOperator(BinaryOperatorToken::Minus),
                RawTokenKind::Number => self.lex_number(start)?,
                RawTokenKind::Whitespace => continue,
                RawTokenKind::Eof => break,
                RawTokenKind::Unknown => {
                    let error = diagnostics::Error::UnknownTokenStart;
                    self.diagnostic_emitter
                        .emit(Diagnostic::new_error(error.clone(), self.span_from(start)));
                    return Err(error);
                }
            };

            tokens.push(Token::new(kind, self.span_from(start)));
        }

        Ok(tokens)
    }

    fn lex_number(&mut self, start: usize) -> diagnostics::Result<TokenKind> {
        let source = self.source_from(start - 1);
        let value = source.parse().map_err(|e| {
            let error = diagnostics::Error::ParseNumber(e);
            self.diagnostic_emitter
                .emit(Diagnostic::new_error(error.clone(), self.span_from(start)));
            error
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

    fn source_from_to_end(&self, start: usize) -> &'a str {
        &self.source[self.source_index(start)..]
    }

    fn span_from(&self, start: usize) -> Span {
        Span::new(start, self.position)
    }
}
