#[derive(Clone, Debug, PartialEq)]
pub struct NumberToken {
    value: u64,
}

impl NumberToken {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }
}
