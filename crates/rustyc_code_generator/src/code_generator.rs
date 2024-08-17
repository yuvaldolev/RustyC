use std::rc::Rc;

use rustyc_hir::Hir;
use rustyc_hir_walker::HirWalker;

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter,
    code_generation_visitor::CodeGenerationVisitor,
};

pub struct CodeGenerator {
    hir: Rc<Hir>,
    instruction_emitter: Aarch64InstructionEmitter,
}

impl CodeGenerator {
    pub fn new(hir: Rc<Hir>) -> Self {
        Self {
            hir,
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        self.instruction_emitter.emit_text_section_directive();

        let walker = HirWalker::new();
        let mut code_generation_visitor = CodeGenerationVisitor::new();
        walker.walk(&self.hir, &mut code_generation_visitor)?;

        Ok(())
    }
}
