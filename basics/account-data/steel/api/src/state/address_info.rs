use steel::*;
use crate::consts::*;

/// Fetch PDA of the account.
pub fn account_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[ACCOUNT], &crate::id())            
}


#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AccountDiscriminator {
    Data = 0
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Data {
    pub name: [u8; 64], // 64 bytes
    pub house_number: u8, // 1 byte
    pub street: [u8; 64], // 64 bytes
    pub city: [u8; 64], // 64 bytes
}

account!(AccountDiscriminator, Data);

