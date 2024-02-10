#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IdentifierToken {
    name: String,
}

impl IdentifierToken {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
