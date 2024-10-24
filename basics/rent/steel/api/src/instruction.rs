use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum RentInstruction {
    InitializeAccount = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InitializeAccount {
    pub name: [u8; 32],
    pub address: [u8; 64],
}

instruction!(RentInstruction, InitializeAccount);


