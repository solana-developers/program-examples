use quasar_lang::prelude::*;

/// Escrow state: records the maker's desired receive amount and the
/// associated mint/token-account addresses.
#[account(discriminator = 1)]
pub struct Escrow {
    pub maker: Address,
    pub mint_a: Address,
    pub mint_b: Address,
    pub maker_ta_b: Address,
    pub receive: u64,
    pub bump: u8,
}
