rustyc_index::newtype_index! {
    pub struct BasicBlockId {
        const START_BASIC_BLOCK = 0;
    }
}

pub struct BasicBlock;

impl BasicBlock {
    pub fn new() -> Self {
        Self
    }
}
