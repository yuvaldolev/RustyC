use rustyc_index::IndexVec;

use crate::{
    basic_block::{BasicBlock, BasicBlockId},
    local::{Local, LocalId},
};

rustyc_index::newtype_index! {
    pub struct BodyId {}
}

pub struct Body {
    basic_blocks: IndexVec<BasicBlockId, BasicBlock>,
    local_declarations: IndexVec<LocalId, Local>,
}

impl Body {
    pub fn new(
        basic_blocks: IndexVec<BasicBlockId, BasicBlock>,
        local_declarations: IndexVec<LocalId, Local>,
    ) -> Self {
        Body {
            basic_blocks,
            local_declarations,
        }
    }
}
