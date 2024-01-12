#[derive(Clone, Debug)]
pub struct Span {
    low: usize,
    high: usize,
}

impl Span {
    pub fn new(low: usize, high: usize) -> Self {
        Self { low, high }
    }

    pub fn new_dummy() -> Self {
        Self::new(0, 0)
    }

    pub fn get_low(&self) -> usize {
        self.low
    }

    pub fn get_high(&self) -> usize {
        self.high
    }
}
