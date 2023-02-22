use anchor_lang::prelude::*;

#[account]
pub struct RentVault {
    pub bump: u8,
}

impl RentVault {
    pub const SEED_PREFIX: &'static str = "rent_vault";
    pub const ACCOUNT_SPACE: usize = 8 + 8;
}
