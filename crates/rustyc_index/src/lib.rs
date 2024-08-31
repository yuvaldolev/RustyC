mod idx;
mod index_slice;
mod index_vec;

pub use rustyc_index_macros::newtype_index;

pub use idx::Idx;
pub use index_slice::IndexSlice;
pub use index_vec::IndexVec;
