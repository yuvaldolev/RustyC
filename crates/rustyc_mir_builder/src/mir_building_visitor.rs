use rustyc_hir_walker::HirVisitor;
use rustyc_mir::Mir;

pub struct MirBuildingVisitor {
    mir: Mir,
    body_builder: BodyBuilder,
}

impl MirBuildingVisitor {
    pub fn new() -> Self {
        Self { mir: Mir::new() }
    }

    pub fn finish(self) -> Mir {
        self.mir
    }
}

impl HirVisitor for MirBuildingVisitor {}
