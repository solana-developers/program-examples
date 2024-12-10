use steel::*;

use super::EscrowAccount;

account!(EscrowAccount, Offer);
// Offer Account
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub bump: u8,
}

impl Offer {
    pub const SEEDS: &'static [u8] = b"offer";
}
