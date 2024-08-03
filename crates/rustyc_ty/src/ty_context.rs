use std::rc::Rc;

use crate::ty::Ty;

// TODO: Instead of using reference counting, return a UID per type
// and allow users to look it up in the `TyContext`, this would be way more efficient.

pub struct TyContext {
    int_type: Rc<Ty>,
}

impl TyContext {
    pub fn new() -> Self {
        Self {
            int_type: Rc::new(Ty::Int),
        }
    }

    pub fn get_int(&self) -> Rc<Ty> {
        Rc::clone(&self.int_type)
    }

    pub fn get_pointer(&self, base: Rc<Ty>) -> Rc<Ty> {
        // TODO: Cache pointers with the same base type.
        Rc::new(Ty::Pointer(base))
    }
}
