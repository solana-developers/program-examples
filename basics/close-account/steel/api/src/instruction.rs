use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum AccountInstruction {
    InitializeAccount = 0,
    CloseAccount = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InitializeAccount {
    pub name: [u8; 32],
}

instruction!(AccountInstruction, InitializeAccount);

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CloseAccount {}

instruction!(AccountInstruction, CloseAccount);
