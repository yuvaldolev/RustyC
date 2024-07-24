#[derive(Debug)]
pub struct VariableProperties {
    offset: i64,
}

impl VariableProperties {
    pub fn new(offset: i64) -> Self {
        Self { offset }
    }

    pub fn get_offset(&self) -> i64 {
        self.offset
    }
}
