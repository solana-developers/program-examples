use {
    crate::state::{Amm, Pool},
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

/// Accounts for creating a new liquidity pool.
///
/// Seeds are based on account addresses: pool = [amm, mint_a, mint_b],
/// pool_authority = [amm, mint_a, mint_b, "authority"],
/// mint_liquidity = [amm, mint_a, mint_b, "liquidity"].
#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(seeds = [b"amm"], bump)]
    pub amm: &'info Account<Amm>,
    #[account(mut, init, payer = payer, seeds = [amm, mint_a, mint_b], bump)]
    pub pool: &'info mut Account<Pool>,
    /// Pool authority PDA — signs for pool token operations.
    #[account(seeds = [amm, mint_a, mint_b, crate::AUTHORITY_SEED], bump)]
    pub pool_authority: &'info UncheckedAccount,
    /// Liquidity token mint — created at a PDA.
    #[account(
        mut,
        init,
        payer = payer,
        seeds = [amm, mint_a, mint_b, crate::LIQUIDITY_SEED],
        bump,
        mint::decimals = 6,
        mint::authority = pool_authority,
    )]
    pub mint_liquidity: &'info mut Account<Mint>,
    pub mint_a: &'info Account<Mint>,
    pub mint_b: &'info Account<Mint>,
    /// Pool's token A account.
    #[account(mut, init_if_needed, payer = payer, token::mint = mint_a, token::authority = pool_authority)]
    pub pool_account_a: &'info mut Account<Token>,
    /// Pool's token B account.
    #[account(mut, init_if_needed, payer = payer, token::mint = mint_b, token::authority = pool_authority)]
    pub pool_account_b: &'info mut Account<Token>,
    #[account(mut)]
    pub payer: &'info Signer,
    pub token_program: &'info Program<Token>,
    pub system_program: &'info Program<System>,
    pub rent: &'info Sysvar<Rent>,
}

#[inline(always)]
pub fn handle_create_pool(accounts: &mut CreatePool) -> Result<(), ProgramError> {
    accounts.pool.amm = *accounts.amm.address();
    accounts.pool.mint_a = *accounts.mint_a.address();
    accounts.pool.mint_b = *accounts.mint_b.address();
    Ok(())
}
