use crate::LocalId;

#[derive(Clone, Debug)]
pub struct VariableReferenceExpression {
    id: LocalId,
}

impl VariableReferenceExpression {
    pub fn new(id: LocalId) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> LocalId {
        self.id
    }
}
