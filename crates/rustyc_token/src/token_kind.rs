use crate::{BinaryOperatorToken, DelimiterToken, NumberToken};

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Number(NumberToken),
    BinaryOperator(BinaryOperatorToken),
    OpenDelimiter(DelimiterToken),
    CloseDelimiter(DelimiterToken),
    Eof,
}
