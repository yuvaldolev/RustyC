// use enum_display::EnumDisplay;

use crate::token::base::Base;

use super::BinaryOperatorToken;

// #[derive(Clone, Debug, EnumDisplay)]
pub enum TokenKind {
    Number(Base, u64),
    BinaryOperator(BinaryOperatorToken),
}
