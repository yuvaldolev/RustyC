mod aarch64_instruction_emitter;
mod code_generation_context;
mod code_generation_visitor;
mod code_generator;
mod expression_generator;
mod function_generator;
mod label_allocator;
mod statement_generator;
mod variable_properties;

pub use code_generator::CodeGenerator;
