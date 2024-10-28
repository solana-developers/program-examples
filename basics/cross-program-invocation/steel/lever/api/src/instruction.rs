use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum LeverInstruction {
    Initialize = 0,
    SwitchPower = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SwitchPower {
    // string max length is 64 bytes
    pub name: [u8; 64],
}

instruction!(LeverInstruction, Initialize);
instruction!(LeverInstruction, SwitchPower);
