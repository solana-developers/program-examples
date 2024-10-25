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
    pub name: [u8; 32]
}

account!(AccountDiscriminator, Data);

