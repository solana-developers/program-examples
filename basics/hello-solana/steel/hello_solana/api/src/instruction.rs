use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum HelloSolanaInstruction {
    Hello = 0
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Hello {}

instruction!(HelloSolanaInstruction, Hello);
