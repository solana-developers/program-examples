use steel::*;

use super::EscrowAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub bump: u8,

    pub _padding: [u8; 7], // Explicit padding to match 960 bits
}

account!(EscrowAccount, Offer);
