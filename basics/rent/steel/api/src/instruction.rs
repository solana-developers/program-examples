use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum RentInstruction {
    CreateSystemAccount = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateSystemAccount {
    pub name: [u8; 32],
    pub address: [u8; 64],
}

instruction!(RentInstruction, CreateSystemAccount);
