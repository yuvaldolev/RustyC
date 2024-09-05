use rustyc_hir::items::FunctionItem;
use rustyc_hir_walker::{HirVisitor, HirWalker};
use rustyc_index::IndexVec;
use rustyc_mir::{Body, BodyId, Mir};

use crate::body_builder::BodyBuilder;

pub struct MirBuildingVisitor {
    bodies: IndexVec<BodyId, Body>,
    body_builder: Option<BodyBuilder>,
}

impl MirBuildingVisitor {
    pub fn new() -> Self {
        Self {
            bodies: IndexVec::new(),
            body_builder: None,
        }
    }

    pub fn finish(self) -> Mir {
        Mir::new(self.bodies)
    }
}

impl HirVisitor for MirBuildingVisitor {
    fn visit_function(
        &mut self,
        function: &FunctionItem,
        walker: &HirWalker,
    ) -> rustyc_diagnostics::Result<()> {
        if let Some(body_builder) = self.body_builder.take() {
            self.bodies.push(body_builder.finish());
        }

        self.body_builder = Some(BodyBuilder::new());

        Ok(())
    }
}
