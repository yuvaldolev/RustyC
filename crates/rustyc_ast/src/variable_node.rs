#[derive(Debug)]
pub struct VariableNode {
    name: char,
}

impl VariableNode {
    pub fn new(name: char) -> Self {
        Self { name }
    }

    pub fn get_name(&self) -> char {
        self.name
    }
}
