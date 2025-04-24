use wasm_bindgen::prelude::*;

// IDEA: convert boolean fields to bit flags
/// Represents a register descriptor.
#[wasm_bindgen]
#[derive(Debug)]
#[non_exhaustive]
pub struct RegisterDescriptor {
    /// The name of the register.
    pub name: &'static str,

    /// The number of bits in the register.
    pub bit_count: usize,

    /// The permissions of the register.
    /// 0 means no permissions, 1 means read-only, 2 means write-only, 3 means read-write.
    pub permissions: u8,

    /// Whether the register is a general-purpose register.
    /// Example: `rax`, `eax`, `ax`, `al`
    pub is_general_purpose: bool,

    /// Whether the register is a control register.
    /// Example: `cr0`, `cr2`, `cr3`, `cr4`
    pub is_control: bool,

    /// Whether the register is a floating-point register.
    /// Example: `st0`, `st1`, `st2`, `st3`, `st4`, `st5`, `st6`, `st7`
    pub is_floating_point: bool,

    /// Whether the register is a vector register.
    /// Example: `xmm0`, `xmm1`, `xmm2`, `xmm3`, `xmm4`, `xmm5`, `xmm6`, `xmm7`
    pub is_vector: bool,
}
