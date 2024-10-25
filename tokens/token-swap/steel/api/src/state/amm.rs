use steel::*;

use super::TokenSwapAccount;

/// Fetch PDA of the amm account.
pub fn amm_pda(id: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[id.as_ref()], &crate::id())
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Amm {
    /// The primary key of the AMM
    pub id: Pubkey,

    /// Account that has admin authority over the AMM
    pub admin: Pubkey,

    /// The LP fee taken on each trade, in basis points
    pub fee: [u8; 2],
}

account!(TokenSwapAccount, Amm);
