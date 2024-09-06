use std::collections::HashMap;

pub struct FunctionLoweringContext {
    locals: HashMap<String, rustyc_hir::LocalId>,
}

impl FunctionLoweringContext {
    pub fn new(locals: HashMap<String, rustyc_hir::LocalId>) -> Self {
        Self { locals }
    }

    pub fn get_hir_local(&self, name: &str) -> Option<rustyc_hir::LocalId> {
        self.locals.get(name).copied()
    }
}
