use wasm_bindgen::prelude::*;

use crate::{memory, register};

/// Represents a register descriptor.
#[wasm_bindgen]
#[derive(Debug)]
#[non_exhaustive]
pub struct EmulatorDescriptor {
    /// The number of bits per byte in the emulator. Usually 8.
    pub bits_per_byte: usize,

    /// The number of memories in the emulator.
    /// 1 for Van Neumann/Princeton architecture, 2 for Harvard architecture.
    pub mem_count: usize,

    /// The number of registers in the emulator. For debug purposes.
    pub reg_count: usize,
}

/// The state of the emulator.
#[wasm_bindgen]
#[derive(PartialEq, Default, Eq, Clone, Copy)]
pub enum State {
    #[default]
    /// The emulator is in a reset state after reset or before the first instruction.
    Reset,
    /// The emulator is in a paused state after stepping, unless it hits a breakpoint, halt instruction or an unknown instruction.
    Paused,
    /// The emulator is in a breakpoint state after hitting a breakpoint.
    BreakPoint,
    /// The emulator is in a halted state after hitting a halt instruction.
    Halted,
    /// The emulator is in an unknown state after hitting an unknown instruction.
    UnknownInstruction,
}

/// The trait for emulators.
pub trait Emulator {
    /// Get the current state of the emulator.
    fn state(&self) -> State;

    /// Step the emulator by one instruction. After stepping, the emulator MUST be in the Paused state, unless it hits a breakpoint, halt instruction or an unknown instruction.
    fn step(&mut self);

    /// Reset the emulator to its initial state. After reset, the emulator MUST be in the Reset state.
    fn reset(&mut self);

    // Ideally this method would be static, but it would make the trait non-dyn-compatible.
    /// Get the description of the emulator.
    fn desc(&self) -> EmulatorDescriptor;

    /// Get a memory of the emulator.
    fn mem(&self, index: usize) -> Option<&[u8]>;

    /// Get a mutable memory of the emulator.
    fn mem_mut(&mut self, index: usize) -> Option<&mut [u8]>;

    // Ideally this method would be static, but it would make the trait non-dyn-compatible.
    // It would also be better if it returned a static reference to the memory descriptor, but it would not be possible to implement a generic emulator.
    /// Get a description of a memory of the emulator.
    fn mem_desc(&self, index: usize) -> Option<memory::MemoryDescriptor>;

    /// Get a register of the emulator.
    /// Returns a slice of bytes representing the register. This allows registers to be of various sizes.
    fn reg(&self, index: usize) -> Option<&[u8]>;

    /// Get a mutable register of the emulator.
    fn reg_mut(&mut self, index: usize) -> Option<&mut [u8]>;

    // Ideally this method would be static, but it would make the trait non-dyn-compatible.
    // It would also be better if it returned a static reference to the memory descriptor, but it would not be possible to implement a generic emulator.
    /// Get a description of a register of the emulator.
    fn reg_desc(&self, index: usize) -> Option<register::RegisterDescriptor>;
}
