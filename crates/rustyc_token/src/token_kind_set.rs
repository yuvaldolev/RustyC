use std::{collections::HashSet, fmt};

use crate::TokenKind;

#[derive(Clone, Debug)]
pub struct TokenKindSet(HashSet<TokenKind>);

impl TokenKindSet {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, token_kind: TokenKind) {
        self.0.insert(token_kind);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn first(&self) -> &TokenKind {
        self.0.iter().next().unwrap()
    }
}

impl Default for TokenKindSet {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TokenKindSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, token_kind) in self.0.iter().enumerate() {
            write!(f, "'{token_kind}'")?;

            if (self.0.len() - 1) != index {
                write!(f, ", ")?;
            }
        }

        Ok(())
    }
}
