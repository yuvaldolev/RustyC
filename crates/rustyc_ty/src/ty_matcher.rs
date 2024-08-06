use std::{cell::RefCell, rc::Rc};

use crate::{Ty, TyContext, TyId};

pub struct TyMatcher {
    ty_context: Rc<RefCell<TyContext>>,
}

impl TyMatcher {
    pub fn new(ty_context: Rc<RefCell<TyContext>>) -> Self {
        Self { ty_context }
    }

    pub fn is_int(&self, ty: TyId) -> bool {
        matches!(self.ty_context.borrow().get(ty), Ty::Int)
    }

    pub fn is_pointer(&self, ty: TyId) -> bool {
        matches!(self.ty_context.borrow().get(ty), Ty::Pointer(_))
    }
}
