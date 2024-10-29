use steel::*;
use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum RentInstruction {
    CreateSystemAccount = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateSystemAccountArgs {
    pub name_len: u32,
    pub name: [u8; STRING_MAX_SIZE],
    pub address_len: u32,
    pub address: [u8; STRING_MAX_SIZE],
}

instruction!(RentInstruction, CreateSystemAccountArgs);
