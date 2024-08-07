use std::{collections::HashSet, fmt};

use crate::TokenCategory;

#[derive(Clone, Debug)]
pub struct TokenCategorySet(HashSet<TokenCategory>);

impl TokenCategorySet {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, token_category: TokenCategory) {
        self.0.insert(token_category);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn first(&self) -> &TokenCategory {
        self.0.iter().next().unwrap()
    }
}

impl Default for TokenCategorySet {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TokenCategorySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, token_category) in self.0.iter().enumerate() {
            write!(f, "'{token_category}'")?;

            if (self.0.len() - 1) != index {
                write!(f, ", ")?;
            }
        }

        Ok(())
    }
}
