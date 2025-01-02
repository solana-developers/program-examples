use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum CounterInstruction {
    Initialize = 0,
    Increment = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Increment {
    pub amount: [u8; 8],
}

instruction!(CounterInstruction, Initialize);
instruction!(CounterInstruction, Increment);
