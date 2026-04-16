use {
    crate::state::{Amm, Pool},
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token, TokenCpi},
};

/// Accounts for depositing liquidity into a pool.
///
/// Seeds reference the amm, mint_a, and mint_b account addresses — these
/// must be provided as separate account inputs.
#[derive(Accounts)]
pub struct DepositLiquidity<'info> {
    #[account(seeds = [b"amm"], bump)]
    pub amm: &'info Account<Amm>,
    #[account(seeds = [amm, mint_a, mint_b], bump)]
    pub pool: &'info Account<Pool>,
    /// Pool authority PDA.
    #[account(seeds = [amm, mint_a, mint_b, crate::AUTHORITY_SEED], bump)]
    pub pool_authority: &'info UncheckedAccount,
    /// Depositor (must be signer to authorise transfers).
    pub depositor: &'info Signer,
    #[account(mut, seeds = [amm, mint_a, mint_b, crate::LIQUIDITY_SEED], bump)]
    pub mint_liquidity: &'info mut Account<Mint>,
    pub mint_a: &'info Account<Mint>,
    pub mint_b: &'info Account<Mint>,
    /// Pool's token A vault.
    #[account(mut)]
    pub pool_account_a: &'info mut Account<Token>,
    /// Pool's token B vault.
    #[account(mut)]
    pub pool_account_b: &'info mut Account<Token>,
    /// Depositor's LP token account.
    #[account(mut, init_if_needed, payer = payer, token::mint = mint_liquidity, token::authority = depositor)]
    pub depositor_account_liquidity: &'info mut Account<Token>,
    /// Depositor's token A account.
    #[account(mut)]
    pub depositor_account_a: &'info mut Account<Token>,
    /// Depositor's token B account.
    #[account(mut)]
    pub depositor_account_b: &'info mut Account<Token>,
    #[account(mut)]
    pub payer: &'info Signer,
    pub token_program: &'info Program<Token>,
    pub system_program: &'info Program<System>,
}

/// Integer square root via Newton's method.
fn isqrt(n: u128) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x as u64
}

#[inline(always)]
pub fn handle_deposit_liquidity(
    accounts: &mut DepositLiquidity, amount_a: u64,
    amount_b: u64,
    bumps: &DepositLiquidityBumps,
) -> Result<(), ProgramError> {
    // Clamp to what the depositor actually has.
    let depositor_a = accounts.depositor_account_a.amount();
    let depositor_b = accounts.depositor_account_b.amount();
    let mut amount_a = if amount_a > depositor_a { depositor_a } else { amount_a };
    let mut amount_b = if amount_b > depositor_b { depositor_b } else { amount_b };

    let pool_a_amount = accounts.pool_account_a.amount();
    let pool_b_amount = accounts.pool_account_b.amount();
    let pool_creation = pool_a_amount == 0 && pool_b_amount == 0;

    if !pool_creation {
        // Adjust amounts to maintain the pool ratio.
        if pool_a_amount > pool_b_amount {
            amount_a = (amount_b as u128)
                .checked_mul(pool_a_amount as u128)
                .ok_or(ProgramError::ArithmeticOverflow)?
                .checked_div(pool_b_amount as u128)
                .ok_or(ProgramError::ArithmeticOverflow)? as u64;
        } else {
            amount_b = (amount_a as u128)
                .checked_mul(pool_b_amount as u128)
                .ok_or(ProgramError::ArithmeticOverflow)?
                .checked_div(pool_a_amount as u128)
                .ok_or(ProgramError::ArithmeticOverflow)? as u64;
        }
    }

    // Compute liquidity = sqrt(amount_a * amount_b).
    let product = (amount_a as u128)
        .checked_mul(amount_b as u128)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    let mut liquidity = isqrt(product);

    // Lock minimum liquidity on first deposit.
    if pool_creation {
        if liquidity < crate::MINIMUM_LIQUIDITY {
            return Err(ProgramError::InsufficientFunds);
        }
        liquidity -= crate::MINIMUM_LIQUIDITY;
    }

    // Transfer token A to the pool.
    accounts.token_program
        .transfer(accounts.depositor_account_a, accounts.pool_account_a, accounts.depositor, amount_a)
        .invoke()?;

    // Transfer token B to the pool.
    accounts.token_program
        .transfer(accounts.depositor_account_b, accounts.pool_account_b, accounts.depositor, amount_b)
        .invoke()?;

    // Mint LP tokens to the depositor (signed by pool authority).
    let bump = [bumps.pool_authority];
    let seeds: &[Seed] = &[
        Seed::from(accounts.amm.address().as_ref()),
        Seed::from(accounts.mint_a.address().as_ref()),
        Seed::from(accounts.mint_b.address().as_ref()),
        Seed::from(crate::AUTHORITY_SEED),
        Seed::from(&bump as &[u8]),
    ];

    accounts.token_program
        .mint_to(
            accounts.mint_liquidity,
            accounts.depositor_account_liquidity,
            accounts.pool_authority,
            liquidity,
        )
        .invoke_signed(seeds)?;

    Ok(())
}
