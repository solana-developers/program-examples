use {
    crate::state::{Amm, Pool},
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token, TokenCpi},
};

/// Accounts for withdrawing liquidity from a pool.
#[derive(Accounts)]
pub struct WithdrawLiquidity<'info> {
    #[account(seeds = [b"amm"], bump)]
    pub amm: &'info Account<Amm>,
    #[account(seeds = [amm, mint_a, mint_b], bump)]
    pub pool: &'info Account<Pool>,
    /// Pool authority PDA.
    #[account(seeds = [amm, mint_a, mint_b, crate::AUTHORITY_SEED], bump)]
    pub pool_authority: &'info UncheckedAccount,
    pub depositor: &'info Signer,
    #[account(mut, seeds = [amm, mint_a, mint_b, crate::LIQUIDITY_SEED], bump)]
    pub mint_liquidity: &'info mut Account<Mint>,
    #[account(mut)]
    pub mint_a: &'info mut Account<Mint>,
    #[account(mut)]
    pub mint_b: &'info mut Account<Mint>,
    #[account(mut)]
    pub pool_account_a: &'info mut Account<Token>,
    #[account(mut)]
    pub pool_account_b: &'info mut Account<Token>,
    #[account(mut)]
    pub depositor_account_liquidity: &'info mut Account<Token>,
    #[account(mut, init_if_needed, payer = payer, token::mint = mint_a, token::authority = depositor)]
    pub depositor_account_a: &'info mut Account<Token>,
    #[account(mut, init_if_needed, payer = payer, token::mint = mint_b, token::authority = depositor)]
    pub depositor_account_b: &'info mut Account<Token>,
    #[account(mut)]
    pub payer: &'info Signer,
    pub token_program: &'info Program<Token>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_withdraw_liquidity(
    accounts: &mut WithdrawLiquidity, amount: u64,
    bumps: &WithdrawLiquidityBumps,
) -> Result<(), ProgramError> {
    let bump = [bumps.pool_authority];
    let seeds: &[Seed] = &[
        Seed::from(accounts.amm.address().as_ref()),
        Seed::from(accounts.mint_a.address().as_ref()),
        Seed::from(accounts.mint_b.address().as_ref()),
        Seed::from(crate::AUTHORITY_SEED),
        Seed::from(&bump as &[u8]),
    ];

    // Compute proportional amounts.
    let total_liquidity = accounts.mint_liquidity.supply() + crate::MINIMUM_LIQUIDITY;

    let amount_a = (amount as u128)
        .checked_mul(accounts.pool_account_a.amount() as u128)
        .ok_or(ProgramError::ArithmeticOverflow)?
        .checked_div(total_liquidity as u128)
        .ok_or(ProgramError::ArithmeticOverflow)? as u64;

    let amount_b = (amount as u128)
        .checked_mul(accounts.pool_account_b.amount() as u128)
        .ok_or(ProgramError::ArithmeticOverflow)?
        .checked_div(total_liquidity as u128)
        .ok_or(ProgramError::ArithmeticOverflow)? as u64;

    // Transfer token A from pool to depositor.
    accounts.token_program
        .transfer(accounts.pool_account_a, accounts.depositor_account_a, accounts.pool_authority, amount_a)
        .invoke_signed(seeds)?;

    // Transfer token B from pool to depositor.
    accounts.token_program
        .transfer(accounts.pool_account_b, accounts.depositor_account_b, accounts.pool_authority, amount_b)
        .invoke_signed(seeds)?;

    // Burn LP tokens.
    accounts.token_program
        .burn(accounts.depositor_account_liquidity, accounts.mint_liquidity, accounts.depositor, amount)
        .invoke()?;

    Ok(())
}
