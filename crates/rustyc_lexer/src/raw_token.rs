use crate::raw_token_kind::RawTokenKind;

pub struct RawToken {
    kind: RawTokenKind,
    length: usize,
}

impl RawToken {
    pub fn new(kind: RawTokenKind, length: usize) -> Self {
        Self { kind, length }
    }

    pub fn get_kind(&self) -> &RawTokenKind {
        &self.kind
    }

    pub fn get_length(&self) -> usize {
        self.length
    }
}
