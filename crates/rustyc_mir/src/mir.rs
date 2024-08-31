use rustyc_index::IndexVec;

use crate::body::{Body, BodyId};

pub struct Mir {
    bodies: IndexVec<BodyId, Body>,
}

impl Mir {
    pub fn new() -> Self {
        Self {
            bodies: IndexVec::new(),
        }
    }
}
