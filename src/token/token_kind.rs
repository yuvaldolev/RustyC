use enum_display::EnumDisplay;

use crate::token::base::Base;

#[derive(Clone, Debug, EnumDisplay)]
pub enum TokenKind {
    Number(Base, u64),
    BinaryOperation(BinaryOperationToken),
}
