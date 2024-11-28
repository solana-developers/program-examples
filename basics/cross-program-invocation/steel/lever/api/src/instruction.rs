use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum LeverInstruction {
    InitializeArgs = 0,
    SwitchPowerArgs = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InitializeArgs {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SwitchPowerArgs {
    pub name_len: u32,
    pub name: [u8; 32],
}

instruction!(LeverInstruction, InitializeArgs);
instruction!(LeverInstruction, SwitchPowerArgs);
