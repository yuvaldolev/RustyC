mod basic_block;
mod body;
mod local;
mod mir;

pub use basic_block::{BasicBlock, BasicBlockId, START_BASIC_BLOCK};
pub use body::{Body, BodyId};
pub use local::{Local, LocalId, RETURN_PLACE};
pub use mir::Mir;
