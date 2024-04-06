use std::mem;

use rustyc_diagnostics::Diagnostic;
use rustyc_span::Span;
use rustyc_token::{BinaryOperatorToken, DelimiterToken, Token, TokenKind};

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

    pub fn lex(mut self) -> rustyc_diagnostics::Result<Vec<Token>> {
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
            }

            match self.token.glue(&next_token) {
                Some(glued) => {
                    self.token = glued;
                }
                None => return Ok(next_token),
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
                RawTokenKind::OpenBrace => TokenKind::OpenDelimiter(DelimiterToken::Brace),
                RawTokenKind::CloseBrace => TokenKind::CloseDelimiter(DelimiterToken::Brace),
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

        Ok(TokenKind::Number(value))
    }

    fn lex_identifier(&self, start: usize) -> TokenKind {
        let source = self.source_from(start);
        TokenKind::Identifier(source.to_owned())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_index_single_source() -> rustyc_diagnostics::Result<()> {
        let lexer = make_empty_lexer()?;

        assert_eq!(lexer.source_index(0), 0);
        assert_eq!(lexer.source_index(10), 10);
        assert_eq!(lexer.source_index(12), 12);
        assert_eq!(lexer.source_index(3), 3);

        Ok(())
    }

    #[test]
    fn test_source_from_empty_in_bounds() -> rustyc_diagnostics::Result<()> {
        let lexer = make_empty_lexer()?;

        let source = lexer.source_from(0);
        assert_eq!(source, "");

        Ok(())
    }

    #[test]
    #[should_panic(expected = "byte index 15 is out of bounds of")]
    fn test_source_from_empty_out_of_bounds() {
        let lexer = make_empty_lexer().unwrap();
        lexer.source_from(15);
    }

    #[test]
    fn test_source_from_non_empty_in_bounds() -> rustyc_diagnostics::Result<()> {
        let lexer = make_non_empty_lexer()?;

        let source = lexer.source_from(0);
        assert_eq!(source, "if");

        Ok(())
    }

    #[test]
    #[should_panic(expected = "byte index 15 is out of bounds of")]
    fn test_source_from_non_empty_out_of_bounds() {
        let lexer = make_non_empty_lexer().unwrap();
        lexer.source_from(15);
    }

    #[test]
    fn test_source_from_non_empty_consumed_in_bounds() -> rustyc_diagnostics::Result<()> {
        let lexer = make_non_empty_consumed_lexer()?;

        let source = lexer.source_from(2);
        assert_eq!(source, " (1) {}");

        Ok(())
    }

    #[test]
    #[should_panic(expected = "byte index 15 is out of bounds of")]
    fn test_source_from_non_empty_consumed_out_of_bounds() {
        let lexer = make_non_empty_consumed_lexer().unwrap();
        lexer.source_from(15);
    }

    #[test]
    fn test_source_from_to_in_bounds() -> rustyc_diagnostics::Result<()> {
        let lexer = make_non_empty_lexer()?;

        let source = lexer.source_from_to(1, 5);
        assert_eq!(source, "f (1");

        Ok(())
    }

    #[test]
    #[should_panic(expected = "byte index 15 is out of bounds of")]
    fn test_source_from_to_out_of_bounds() {
        let lexer = make_non_empty_lexer().unwrap();
        lexer.source_from_to(1, 15);
    }

    #[test]
    fn test_span_from_empty() -> rustyc_diagnostics::Result<()> {
        let lexer = make_empty_lexer()?;

        let span = lexer.span_from(0);
        assert_eq!(span.get_low(), 0);
        assert_eq!(span.get_high(), 0);

        Ok(())
    }

    #[test]
    fn test_span_from_non_empty() -> rustyc_diagnostics::Result<()> {
        let lexer = make_non_empty_lexer()?;

        let span = lexer.span_from(0);
        assert_eq!(span.get_low(), 0);
        assert_eq!(span.get_high(), 2);

        Ok(())
    }

    #[test]
    fn test_span_from_non_empty_consumed() -> rustyc_diagnostics::Result<()> {
        let lexer = make_non_empty_consumed_lexer()?;

        let span = lexer.span_from(0);
        assert_eq!(span.get_low(), 0);
        assert_eq!(span.get_high(), 9);

        Ok(())
    }

    fn make_empty_lexer() -> rustyc_diagnostics::Result<Lexer<'static>> {
        make_lexer("")
    }

    fn make_non_empty_lexer() -> rustyc_diagnostics::Result<Lexer<'static>> {
        make_lexer("if (1) {}")
    }

    fn make_non_empty_consumed_lexer() -> rustyc_diagnostics::Result<Lexer<'static>> {
        make_consumed_lexer("if (1) {}")
    }

    fn make_consumed_lexer(source: &str) -> rustyc_diagnostics::Result<Lexer> {
        let mut lexer = make_lexer(source)?;
        lex_all(&mut lexer)?;

        Ok(lexer)
    }

    fn make_lexer(source: &str) -> rustyc_diagnostics::Result<Lexer> {
        let lexer = Lexer::new(source)?;
        Ok(lexer)
    }

    fn lex_all(lexer: &mut Lexer) -> rustyc_diagnostics::Result<()> {
        loop {
            match lexer.token.get_kind() {
                TokenKind::Eof => break,
                _ => lexer.bump(true)?,
            };
        }

        Ok(())
    }
}
