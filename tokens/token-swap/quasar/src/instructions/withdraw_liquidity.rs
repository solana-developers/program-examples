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

impl WithdrawLiquidity<'_> {
    #[inline(always)]
    pub fn withdraw_liquidity(
        &mut self,
        amount: u64,
        bumps: &WithdrawLiquidityBumps,
    ) -> Result<(), ProgramError> {
        let bump = [bumps.pool_authority];
        let seeds: &[Seed] = &[
            Seed::from(self.amm.address().as_ref()),
            Seed::from(self.mint_a.address().as_ref()),
            Seed::from(self.mint_b.address().as_ref()),
            Seed::from(crate::AUTHORITY_SEED),
            Seed::from(&bump as &[u8]),
        ];

        // Compute proportional amounts.
        let total_liquidity = self.mint_liquidity.supply() + crate::MINIMUM_LIQUIDITY;

        let amount_a = (amount as u128)
            .checked_mul(self.pool_account_a.amount() as u128)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(total_liquidity as u128)
            .ok_or(ProgramError::ArithmeticOverflow)? as u64;

        let amount_b = (amount as u128)
            .checked_mul(self.pool_account_b.amount() as u128)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(total_liquidity as u128)
            .ok_or(ProgramError::ArithmeticOverflow)? as u64;

        // Transfer token A from pool to depositor.
        self.token_program
            .transfer(self.pool_account_a, self.depositor_account_a, self.pool_authority, amount_a)
            .invoke_signed(seeds)?;

        // Transfer token B from pool to depositor.
        self.token_program
            .transfer(self.pool_account_b, self.depositor_account_b, self.pool_authority, amount_b)
            .invoke_signed(seeds)?;

        // Burn LP tokens.
        self.token_program
            .burn(self.depositor_account_liquidity, self.mint_liquidity, self.depositor, amount)
            .invoke()?;

        Ok(())
    }
}
