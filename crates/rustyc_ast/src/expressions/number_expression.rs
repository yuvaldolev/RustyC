#[derive(Clone, Debug)]
pub struct NumberExpression {
    value: u64,
}

impl NumberExpression {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }
}
