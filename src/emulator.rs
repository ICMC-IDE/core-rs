use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum State {
    Paused,
    BreakPoint,
    Halted,
    UnknownInstruction,
}

pub trait Emulator {
    fn new() -> Self;
    fn state(&self) -> State;
    fn step(&mut self);
    fn reset(&mut self);
}
