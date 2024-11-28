use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum HandInstruction {
    PullLeverArgs = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PullLeverArgs {
    pub name_len: u32,
    pub name: [u8; 32],
}

instruction!(HandInstruction, PullLeverArgs);
