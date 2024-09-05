use rustyc_index::IndexVec;
use rustyc_mir::{BasicBlock, BasicBlockId, Body};

pub struct BodyBuilder {
    blocks: IndexVec<BasicBlockId, BasicBlock>,
}

impl BodyBuilder {
    pub fn new() -> Self {
        let mut blocks: IndexVec<BasicBlockId, BasicBlock> = IndexVec::new();
        blocks[0] = BasicBlock::new();
    }

    pub fn finish(self) -> Body {
        Body::new(IndexVec::new(), IndexVec::new())
    }
}
