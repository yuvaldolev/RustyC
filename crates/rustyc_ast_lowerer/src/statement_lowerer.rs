use std::rc::Rc;

pub struct StatementLowerer {
    statement: Rc<rustyc_ast::Statement>,
}

impl StatementLowerer {
    pub fn new(statement: Rc<rustyc_ast::Statement>) -> Self {
        Self { statement }
    }

    pub fn lower(self) -> Rc<rustyc_hir::Statement> {
        Rc::new(rustyc_hir::Statement)
    }
}
