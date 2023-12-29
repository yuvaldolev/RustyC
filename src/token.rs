use enum_display::EnumDisplay;

#[derive(Clone, Debug, EnumDisplay)]
pub enum Token {
    Punctuator(char),
    Number(u64),
}
