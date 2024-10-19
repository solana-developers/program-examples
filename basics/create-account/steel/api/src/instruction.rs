use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum CreateAccountInstruction {
    InitializeNewAccount = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InitializeNewAccount {}

instruction!(CreateAccountInstruction, InitializeNewAccount);

