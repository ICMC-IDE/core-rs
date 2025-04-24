use wasm_bindgen::prelude::*;

/// Represents a memory descriptor.
#[wasm_bindgen]
#[derive(Debug)]
#[non_exhaustive]
pub struct MemoryDescriptor {
    /// The number of bits per byte.
    /// When the address is incremented by one, how many bits are shifted to the left.
    pub bits_per_byte: usize,
    /// The size of the memory in bytes.
    pub size: usize,
    /// The base address of the memory.
    pub base_address: usize,
    /// The permissions of the memory.
    /// 0 means no permissions, 1 means read-only, 2 means write-only, 3 means read-write.
    pub permissions: u8,
}
