pub struct Aarch64InstructionEmitter;

// TODO: This should be moved to a platform abstraction layer that wraps
// instruction emitters as well as other platform attributes such as the
// calling convention.
const FUNCTION_PARAMETER_REGISTERS: [&str; 8] = ["x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"];

// TODO: All the formatting in this file can probably be done with an Arena allocator
// instead of repeatedly allocating Strings in each `format!` invocation.
impl Aarch64InstructionEmitter {
    pub fn new() -> Self {
        Self
    }

    pub fn emit_move_registers(&self, source: &str, destination: &str) {
        Self::emit_instruction(format!("mov {destination}, {source}").as_str());
    }

    pub fn emit_move_signed_immediate_to_register(&self, source: i64, destination: &str) {
        Self::emit_instruction(format!("mov {destination}, #{source}").as_str());
    }

    pub fn emit_add_registers(&self, register_a: &str, register_b: &str, destination: &str) {
        Self::emit_instruction(format!("add {destination}, {register_a}, {register_b}").as_str());
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

    pub fn emit_load(&self, source: &str, destination: &str) {
        Self::emit_instruction(format!("ldr {destination}, [{source}]").as_str());
    }

    pub fn emit_load_offset(&self, source: &str, offset: i64, destination: &str) {
        Self::emit_instruction(format!("ldr {destination}, [{source}, #{offset}]").as_str());
    }

    pub fn emit_store(&self, source: &str, destination: &str) {
        Self::emit_instruction(format!("str {source}, [{destination}]").as_str());
    }

    pub fn emit_store_offset(&self, source: &str, destination: &str, offset: i64) {
        Self::emit_instruction(format!("str {source}, [{destination}, #{offset}]").as_str());
    }

    pub fn emit_return(&self) {
        Self::emit_instruction("ret");
    }

    pub fn emit_branch(&self, target: &str) {
        Self::emit_instruction(format!("b {target}").as_str());
    }

    pub fn emit_branch_equals(&self, target: &str) {
        Self::emit_instruction(format!("beq {target}").as_str());
    }

    pub fn emit_branch_link(&self, target: &str) {
        Self::emit_instruction(format!("bl {target}").as_str());
    }

    pub fn emit_conditional_set(&self, condition: &str) {
        self.emit_comparison("x0", "x1");
        Self::emit_instruction(format!("cset x0, {condition}").as_str());
    }

    pub fn emit_comparison(&self, a: &str, b: &str) {
        Self::emit_instruction(format!("cmp {a}, {b}").as_str());
    }

    pub fn emit_label(&self, label: &str) {
        println!("{label}:");
    }

    pub fn emit_global(&self, symbol: &str) {
        println!(".global {symbol}");
    }

    pub fn emit_text_section_directive(&self) {
        println!(".text");
    }

    pub fn emit_item_separator(&self) {
        println!();
    }

    pub fn get_function_parameter_register(&self, index: usize) -> &str {
        FUNCTION_PARAMETER_REGISTERS[index]
    }

    fn emit_instruction(instruction: &str) {
        println!("  {}", instruction);
    }
}
