use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum RentInstruction {
    CreateSystemAccountArgs = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateSystemAccountArgs {
    pub name_len: u32,
    pub name: [u8; 32],  
    pub address_len: u32,
    pub address: [u8; 32],  
}

instruction!(RentInstruction, CreateSystemAccountArgs);