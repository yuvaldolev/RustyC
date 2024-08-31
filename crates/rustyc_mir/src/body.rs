use rustyc_index::IndexVec;

use crate::basic_block::{BasicBlock, BasicBlockId};

rustyc_index::newtype_index! {
    pub struct BodyId {}
}

pub struct Body {
    basic_blocks: IndexVec<BasicBlockId, BasicBlock>,
}
