#[derive(Debug)]
pub struct NumberNode {
    value: u64,
}

impl NumberNode {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }
}
