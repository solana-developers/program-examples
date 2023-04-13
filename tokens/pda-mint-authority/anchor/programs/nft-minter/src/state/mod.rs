use anchor_lang::prelude::*;

#[account]
pub struct MintAuthorityPda {
    pub bump: u8,
}

impl MintAuthorityPda {
    pub const SEED_PREFIX: &'static str = "mint_authority";
    pub const SIZE: usize = 8 + 8;
}
