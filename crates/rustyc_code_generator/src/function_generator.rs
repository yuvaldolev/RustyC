use std::{cell::RefCell, rc::Rc};

use rustyc_ast::FunctionItem;

use crate::{
    aarch64_instruction_emitter::Aarch64InstructionEmitter, block_generator::BlockGenerator,
    function::Function, label_allocator::LabelAllocator,
};

pub struct FunctionGenerator {
    function: Function,
    label_allocator: Rc<RefCell<LabelAllocator>>,
    instruction_emitter: Aarch64InstructionEmitter,
}

impl FunctionGenerator {
    pub fn new(function: Rc<FunctionItem>) -> Self {
        let label_allocator = Rc::new(RefCell::new(LabelAllocator::new(
            function.get_name().to_owned(),
        )));

        Self {
            function: Function::new(function),
            label_allocator,
            instruction_emitter: Aarch64InstructionEmitter::new(),
        }
    }

    pub fn generate(self) -> rustyc_diagnostics::Result<()> {
        self.generate_prologue();

        self.generate_push_parameters_to_stack();

        let block_generator = BlockGenerator::new(
            self.function.get_ast().get_body(),
            self.function.get_local_variables(),
            Rc::clone(&self.label_allocator),
        );
        block_generator.generate()?;

        self.generate_epilogue();

        Ok(())
    }

    fn generate_prologue(&self) {
        // TODO: This logic is only relevant to macOS.
        // This would need to be abstracted somehow when adding support
        // for other platforms.
        let function_name = format!("_{}", self.function.get_ast().get_name());
        self.instruction_emitter.emit_global(&function_name);
        self.instruction_emitter.emit_label(&function_name);

        self.instruction_emitter.emit_push_pair("fp", "lr");
        self.instruction_emitter.emit_move("sp", "fp");
        self.instruction_emitter.emit_subtract(
            "sp",
            self.function.get_stack_size().to_string().as_str(),
            "sp",
        );
    }

    fn generate_push_parameters_to_stack(&self) {
        for (index, parameter) in self.function.get_ast().get_parameters().iter().enumerate() {
            // TODO: Emit an error if the variable is not found, instead of panicking.
            self.instruction_emitter.emit_variable_write(
                self.function.get_local_variables().get(parameter).unwrap(),
                self.instruction_emitter
                    .get_function_parameter_register(index),
            );
        }
    }

    fn generate_epilogue(&self) {
        self.instruction_emitter.emit_label(
            self.label_allocator
                .borrow()
                .allocate_global("return")
                .as_str(),
        );

        self.instruction_emitter.emit_move("fp", "sp");
        self.instruction_emitter.emit_pop_pair("fp", "lr");
        self.instruction_emitter.emit_return();
    }
}
