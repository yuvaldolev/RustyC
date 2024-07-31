use std::rc::Rc;

pub enum Ty {
    Int,
    // TODO: Should rc::Weak be used here?
    Pointer(Rc<Ty>),
}
