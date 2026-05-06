use quasar_lang::prelude::*;

/// State for the fundraiser: records the maker, target mint, amounts, and timing.
#[account(discriminator = 1)]
pub struct Fundraiser {
    pub maker: Address,
    pub mint_to_raise: Address,
    pub amount_to_raise: u64,
    pub current_amount: u64,
    pub time_started: i64,
    pub duration: u16,
    pub bump: u8,
}

/// Tracks how much a specific contributor has given.
#[account(discriminator = 2)]
pub struct Contributor {
    pub amount: u64,
}
