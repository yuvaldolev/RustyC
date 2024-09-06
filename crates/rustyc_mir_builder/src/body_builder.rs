use rustyc_index::IndexVec;
use rustyc_mir::{BasicBlock, BasicBlockId, Body, Local, LocalId};

pub struct BodyBuilder {
    basic_blocks: IndexVec<BasicBlockId, BasicBlock>,
    local_declarations: IndexVec<LocalId, Local>,
}

impl BodyBuilder {
    pub fn new() -> Self {
        let mut body_builder = BodyBuilder {
            basic_blocks: IndexVec::new(),
            local_declarations: IndexVec::new(),
        };

        assert_eq!(
            body_builder.start_new_block(),
            rustyc_mir::START_BASIC_BLOCK
        );
        assert_eq!(
            body_builder.local_declarations.push(Local::new()),
            rustyc_mir::RETURN_PLACE,
        );

        body_builder
    }

    pub fn finish(self) -> Body {
        Body::new(self.basic_blocks, IndexVec::new())
    }

    fn start_new_block(&mut self) -> BasicBlockId {
        self.basic_blocks.push(BasicBlock::new())
    }
}
