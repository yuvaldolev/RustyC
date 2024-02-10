#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IdentifierToken {
    name: char,
}

impl IdentifierToken {
    pub fn new(name: char) -> Self {
        Self { name }
    }

    pub fn get_name(&self) -> char {
        self.name
    }
}
