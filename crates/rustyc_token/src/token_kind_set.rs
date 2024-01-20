use std::{collections::HashSet, fmt};

use crate::TokenKind;

#[derive(Clone, Debug)]
pub struct TokenKindSet(HashSet<TokenKind>);

impl TokenKindSet {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn insert(&mut self, token_kind: TokenKind) {
        self.0.insert(token_kind);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl Default for TokenKindSet {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TokenKindSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted_items_count: usize = 0;

        for token_kind in self.0.iter() {
            write!(f, "'{token_kind}'")?;

            if (self.0.len() - 1) != formatted_items_count {
                write!(f, ", ")?;
            }

            formatted_items_count += 1;
        }

        Ok(())
    }
}
