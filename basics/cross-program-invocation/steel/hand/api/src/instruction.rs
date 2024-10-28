use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum HandInstruction {
    PullLever = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PullLever {
    pub name: [u8; 32],
}

instruction!(HandInstruction, PullLever);
