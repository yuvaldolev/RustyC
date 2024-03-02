use crate::variable_properties::VariableProperties;

pub struct Aarch64InstructionEmitter;

impl Aarch64InstructionEmitter {
    pub fn new() -> Self {
        Self
    }

    pub fn emit_move(&self, source: &str, destination: &str) {
        Self::emit_instruction(format!("mov {destination}, {source}").as_str());
    }

    pub fn emit_add(&self, source_a: &str, source_b: &str, destination: &str) {
        Self::emit_instruction(format!("add {destination}, {source_a}, {source_b}").as_str());
    }

    pub fn emit_subtract(&self, source_a: &str, source_b: &str, destination: &str) {
        Self::emit_instruction(format!("sub {destination}, {source_a}, {source_b}").as_str());
    }

    pub fn emit_multiply(&self, source_a: &str, source_b: &str, destination: &str) {
        Self::emit_instruction(format!("mul {destination}, {source_a}, {source_b}").as_str());
    }

    pub fn emit_divide(&self, source_a: &str, source_b: &str, destination: &str) {
        Self::emit_instruction(format!("sdiv {destination}, {source_a}, {source_b}").as_str());
    }

    pub fn emit_negate(&self, source: &str, destination: &str) {
        Self::emit_instruction(format!("neg {destination}, {source}").as_str());
    }

    pub fn emit_push(&self, register: &str) {
        self.emit_push_pair(register, "xzr");
    }

    pub fn emit_push_pair(&self, register1: &str, register2: &str) {
        Self::emit_instruction(format!("stp {register1}, {register2}, [sp, #-0x10]!").as_str());
    }

    pub fn emit_pop(&self, register: &str) {
        self.emit_pop_pair(register, "xzr");
    }

    pub fn emit_pop_pair(&self, register1: &str, register2: &str) {
        Self::emit_instruction(format!("ldp {register1}, {register2}, [sp], #0x10").as_str());
    }

    pub fn emit_return(&self) {
        Self::emit_instruction("ret");
    }

    pub fn emit_branch(&self, target: &str) {
        Self::emit_instruction(format!("b {target}").as_str());
    }

    pub fn emit_variable_read(&self, variable: &VariableProperties) {
        Self::emit_instruction(format!("ldr x0, [fp, #{}]", variable.get_offset()).as_str());
    }

    pub fn emit_variable_write(&self, variable: &VariableProperties) {
        Self::emit_instruction(format!("str x0, [fp, #{}]", variable.get_offset()).as_str());
    }

    pub fn emit_comparison(&self, condition: &str) {
        Self::emit_instruction("cmp x0, x1");
        Self::emit_instruction(format!("cset x0, {condition}").as_str());
    }

    pub fn emit_label(&self, label: &str) {
        println!("{label}:");
    }

    pub fn emit_global(&self, symbol: &str) {
        println!(".global {symbol}");
    }

    fn emit_instruction(instruction: &str) {
        println!("  {}", instruction);
    }
}
