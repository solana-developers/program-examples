mod mint_authority;

pub use mint_authority::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    MintAuthorityPda = 0,
}

/// Fetch PDA of the mint authority account.
pub fn mint_authority_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[MINT_AUTHORITY], &crate::id())
}
