use std::rc::Rc;

use crate::ty::Ty;

pub struct TyContext {
    int_type: Rc<Ty>,
}

impl TyContext {
    pub fn new() -> Self {
        Self {
            int_type: Rc::new(Ty::Int),
        }
    }

    pub fn get_int_type(&self) -> Rc<Ty> {
        Rc::clone(&self.int_type)
    }
}
