use steel::*;

use crate::consts::ACCOUNT_DATA_SEED;

/// Account type discriminator
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AddressInfoDescriminator {
    AddressInfoData = 0,
}

/// Fetch PDA of the data account.
pub fn address_info_pda(authority: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[ACCOUNT_DATA_SEED, authority.as_ref()], &crate::id())
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AddressInfoData {
    // Max length 64 bytes
    pub name: [u8; 64],
    pub house_number: u8,
    // Max length 64 bytes
    pub street: [u8; 64],
    // Max length 64 bytes
    pub city: [u8; 64],
    pub bump: u8,
}

account!(AddressInfoDescriminator, AddressInfoData);
