use super::{BinaryOperatorToken, DelimiterToken, NumberToken};

#[derive(Clone, Debug)]
pub enum TokenKind {
    Number(NumberToken),
    BinaryOperator(BinaryOperatorToken),
    OpenDelimiter(DelimiterToken),
    CloseDelimiter(DelimiterToken),
    Eof,
}
