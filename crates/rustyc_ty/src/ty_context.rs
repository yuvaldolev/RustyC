use std::collections::HashMap;

use crate::{ty::Ty, TyId};

pub struct TyContext {
    types: HashMap<TyId, Ty>,
    next_id: TyId,
}

impl TyContext {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            next_id: TyId::new(0),
        }
    }

    pub fn get(&self, id: TyId) -> &Ty {
        self.types.get(&id).unwrap()
    }

    pub fn register(&mut self, ty: Ty) -> TyId {
        if let Some((id, _)) = self.types.iter().find(|(_, v)| **v == ty) {
            *id
        } else {
            let id = self.next_id();
            self.types.insert(id, ty);

            id
        }
    }

    fn next_id(&mut self) -> TyId {
        let id = self.next_id;
        self.next_id = TyId::new(id.get() + 1);

        id
    }
}
