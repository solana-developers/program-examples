use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum HandInstruction {
    PullLever = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PullLever {
    // string max length is 64 bytes
    pub name: [u8; 64],
}

instruction!(HandInstruction, PullLever);
