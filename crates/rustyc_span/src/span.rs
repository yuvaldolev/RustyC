use std::cmp;

#[derive(Clone, Debug, Eq, PartialEq)]
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

    pub fn to(&self, end: &Self) -> Self {
        Self::new(cmp::min(self.low, end.low), cmp::max(self.high, end.high))
    }

    pub fn get_low(&self) -> usize {
        self.low
    }

    pub fn get_high(&self) -> usize {
        self.high
    }
}
