use quasar_lang::prelude::*;

/// Automated Market Maker configuration.
///
/// Stores the AMM identifier, admin, and fee (in basis points).
#[account(discriminator = 100)]
pub struct Amm {
    /// Unique identifier for this AMM.
    pub id: Address,
    /// Admin authority.
    pub admin: Address,
    /// LP fee in basis points (e.g. 30 = 0.3%).
    pub fee: u16,
}

/// Liquidity pool linking an AMM to a pair of token mints.
#[account(discriminator = 101)]
pub struct Pool {
    /// The AMM this pool belongs to.
    pub amm: Address,
    /// Mint of token A.
    pub mint_a: Address,
    /// Mint of token B.
    pub mint_b: Address,
}
