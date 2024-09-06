use crate::LocalId;

#[derive(Clone, Debug)]
pub struct VariableExpression {
    id: LocalId,
}

impl VariableExpression {
    pub fn new(id: LocalId) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> LocalId {
        self.id
    }
}
