use std::mem;

use rustyc_diagnostics::Diagnostic;
use rustyc_span::Span;
use rustyc_token::{
    BinaryOperatorToken, DelimiterToken, IdentifierToken, NumberToken, Token, TokenKind,
};

use crate::{raw_token_cursor::RawTokenCursor, raw_token_kind::RawTokenKind};

pub struct Lexer<'a> {
    source: &'a str,
    cursor: RawTokenCursor<'a>,
    token: Token,
    start_position: usize,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> rustyc_diagnostics::Result<Self> {
        let mut lexer = Self {
            source,
            cursor: RawTokenCursor::new(source),
            token: Token::new_eof(),
            start_position: 0,
            position: 0,
        };

        lexer.bump(false)?;

        Ok(lexer)
    }

    pub fn lex(&mut self) -> rustyc_diagnostics::Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            match self.token.get_kind() {
                TokenKind::Eof => break,
                _ => tokens.push(self.bump(true)?),
            }
        }

        Ok(tokens)
    }

    fn bump(&mut self, glue: bool) -> rustyc_diagnostics::Result<Token> {
        let next_token = if glue {
            self.glue_token()?
        } else {
            let (token, _) = self.lex_token()?;
            token
        };

        let this_token = mem::replace(&mut self.token, next_token);

        Ok(this_token)
    }

    fn glue_token(&mut self) -> rustyc_diagnostics::Result<Token> {
        loop {
            let (next_token, next_token_preceded_by_whitespace) = self.lex_token()?;

            if next_token_preceded_by_whitespace {
                return Ok(next_token);
            } else if let Some(glued) = self.token.glue(&next_token) {
                self.token = glued;
            } else {
                return Ok(next_token);
            }
        }
    }

    fn lex_token(&mut self) -> rustyc_diagnostics::Result<(Token, bool)> {
        let mut preceded_by_whitespace = false;

        loop {
            let raw_token = self.cursor.next();

            let start = self.position;
            self.position += raw_token.get_length();

            let kind = match raw_token.get_kind() {
                RawTokenKind::Equal => TokenKind::Equal,
                RawTokenKind::LessThan => TokenKind::LessThan,
                RawTokenKind::GreaterThan => TokenKind::GreaterThan,
                RawTokenKind::Bang => TokenKind::Not,
                RawTokenKind::Plus => TokenKind::BinaryOperator(BinaryOperatorToken::Plus),
                RawTokenKind::Minus => TokenKind::BinaryOperator(BinaryOperatorToken::Minus),
                RawTokenKind::Star => TokenKind::BinaryOperator(BinaryOperatorToken::Star),
                RawTokenKind::Slash => TokenKind::BinaryOperator(BinaryOperatorToken::Slash),
                RawTokenKind::OpenParenthesis => {
                    TokenKind::OpenDelimiter(DelimiterToken::Parenthesis)
                }
                RawTokenKind::CloseParenthesis => {
                    TokenKind::CloseDelimiter(DelimiterToken::Parenthesis)
                }
                RawTokenKind::Semicolon => TokenKind::Semicolon,
                RawTokenKind::Number => self.lex_number(start)?,
                RawTokenKind::Identifier => self.lex_identifier(start),
                RawTokenKind::Whitespace => {
                    preceded_by_whitespace = true;
                    continue;
                }
                RawTokenKind::Eof => TokenKind::Eof,
                RawTokenKind::Unknown => {
                    return Err(Diagnostic::new_error(
                        rustyc_diagnostics::Error::UnknownTokenStart,
                        self.span_from(start),
                    ))
                }
            };

            return Ok((
                Token::new(kind, self.span_from(start)),
                preceded_by_whitespace,
            ));
        }
    }

    fn lex_number(&self, start: usize) -> rustyc_diagnostics::Result<TokenKind> {
        let source = self.source_from(start);
        let value = source.parse().map_err(|e| {
            Diagnostic::new_error(
                rustyc_diagnostics::Error::ParseNumber(e),
                self.span_from(start),
            )
        })?;

        Ok(TokenKind::Number(NumberToken::new(value)))
    }

    fn lex_identifier(&self, start: usize) -> TokenKind {
        let source = self.source_from(start);
        TokenKind::Identifier(IdentifierToken::new(source.to_owned()))
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
