#[derive(Clone, Debug)]
pub struct VariableReferenceExpression {
    identifier: String,
}

impl VariableReferenceExpression {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}
