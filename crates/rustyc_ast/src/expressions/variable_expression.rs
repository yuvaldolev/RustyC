#[derive(Clone, Debug)]
pub struct VariableExpression {
    name: String,
}

impl VariableExpression {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
