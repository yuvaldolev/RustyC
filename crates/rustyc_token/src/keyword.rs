use enum_display::EnumDisplay;

#[derive(Clone, Debug, EnumDisplay, Eq, Hash, PartialEq)]
pub enum Keyword {
    Return,
    If,
    Else,
    For,
    While,
}
