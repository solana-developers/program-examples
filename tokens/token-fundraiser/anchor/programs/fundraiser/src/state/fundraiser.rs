use anchor_lang::prelude::*;

#[account]
pub struct Fundraiser {
    pub maker: Pubkey,
    pub mint_to_raise: Pubkey,
    pub amount_to_raise: u64,
    pub bump: u8,
}

impl Space for Fundraiser {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}