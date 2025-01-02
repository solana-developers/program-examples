use steel::*;

use crate::consts::OFFER_SEED;

use super::EscrowAccount;

/// Fetch PDA of the counter account.
pub fn offer_pda(maker: Pubkey, id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[OFFER_SEED, maker.as_ref(), id.to_le_bytes().as_ref()],
        &crate::id(),
    )
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Offer {
    pub id: [u8; 8],
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: [u8; 8],
    pub bump: u8,
}

account!(EscrowAccount, Offer);
