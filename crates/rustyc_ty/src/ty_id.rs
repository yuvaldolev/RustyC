#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct TyId {
    value: u64,
}

impl TyId {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn get(&self) -> u64 {
        self.value
    }
}
