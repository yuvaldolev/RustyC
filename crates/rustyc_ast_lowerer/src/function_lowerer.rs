use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rustyc_index::IndexVec;
use rustyc_ty::TyContext;

use crate::{block_lowerer::BlockLowerer, function_lowering_context::FunctionLoweringContext};

pub struct FunctionLowerer {
    function: Rc<rustyc_ast::items::FunctionItem>,
    ty_context: Rc<RefCell<TyContext>>,
}

impl FunctionLowerer {
    pub fn new(
        function: Rc<rustyc_ast::items::FunctionItem>,
        ty_context: Rc<RefCell<TyContext>>,
    ) -> Self {
        Self {
            function,
            ty_context,
        }
    }

    pub fn lower(self) -> rustyc_diagnostics::Result<Rc<rustyc_hir::items::FunctionItem>> {
        let mut hir_locals: IndexVec<rustyc_hir::LocalId, rustyc_hir::Local> = IndexVec::new();
        let mut name_to_hir_local: HashMap<String, rustyc_hir::LocalId> = HashMap::new();
        let hir_parameters = Self::lower_locals(
            self.function.get_parameters(),
            &mut hir_locals,
            &mut name_to_hir_local,
        );
        let hir_local_variables = Self::lower_locals(
            self.function.get_local_variables(),
            &mut hir_locals,
            &mut name_to_hir_local,
        );

        let body = self.lower_block(
            Rc::new(FunctionLoweringContext::new(name_to_hir_local)),
            self.function.get_body(),
        )?;

        Ok(Rc::new(rustyc_hir::items::FunctionItem::new(
            self.function.get_name().to_owned(),
            hir_locals,
            hir_parameters,
            body,
            hir_local_variables,
        )))
    }

    fn lower_locals(
        ast_locals: &[String],
        hir_locals: &mut IndexVec<rustyc_hir::LocalId, rustyc_hir::Local>,
        name_to_hir_local: &mut HashMap<String, rustyc_hir::LocalId>,
    ) -> Vec<rustyc_hir::LocalId> {
        let mut lowered_locals: Vec<rustyc_hir::LocalId> = Vec::new();

        for ast_local in ast_locals.iter() {
            if name_to_hir_local.contains_key(ast_local) {
                // TODO: Find a way to return an error - this should be once variable declarations
                // are mandatory.
                continue;
            }

            let hir_local = hir_locals.push(rustyc_hir::Local::new());
            name_to_hir_local.insert(ast_local.clone(), hir_local);
            lowered_locals.push(hir_local);
        }

        lowered_locals
    }

    fn lower_block(
        &self,
        context: Rc<FunctionLoweringContext>,
        block: Rc<rustyc_ast::Block>,
    ) -> rustyc_diagnostics::Result<Rc<rustyc_hir::Block>> {
        let block_lowerer = BlockLowerer::new(context, block, Rc::clone(&self.ty_context));
        block_lowerer.lower()
    }
}
