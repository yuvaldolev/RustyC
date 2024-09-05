mod basic_block;
mod body;
mod local;
mod mir;

pub use basic_block::{BasicBlock, BasicBlockId};
pub use body::{Body, BodyId};
pub use mir::Mir;
