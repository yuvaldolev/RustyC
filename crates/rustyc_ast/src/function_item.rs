use crate::block::Block;

pub struct FunctionItem {
    body: Block,
    locals: Vec<String>,
}
