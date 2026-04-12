use {
    crate::state::{Amm, Pool},
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token, TokenCpi},
};

/// Accounts for swapping tokens using the constant-product formula.
#[derive(Accounts)]
pub struct SwapExactTokensForTokens<'info> {
    #[account(seeds = [b"amm"], bump)]
    pub amm: &'info Account<Amm>,
    #[account(seeds = [amm, mint_a, mint_b], bump)]
    pub pool: &'info Account<Pool>,
    /// Pool authority PDA.
    #[account(seeds = [amm, mint_a, mint_b, crate::AUTHORITY_SEED], bump)]
    pub pool_authority: &'info UncheckedAccount,
    pub trader: &'info Signer,
    pub mint_a: &'info Account<Mint>,
    pub mint_b: &'info Account<Mint>,
    #[account(mut)]
    pub pool_account_a: &'info mut Account<Token>,
    #[account(mut)]
    pub pool_account_b: &'info mut Account<Token>,
    #[account(mut, init_if_needed, payer = payer, token::mint = mint_a, token::authority = trader)]
    pub trader_account_a: &'info mut Account<Token>,
    #[account(mut, init_if_needed, payer = payer, token::mint = mint_b, token::authority = trader)]
    pub trader_account_b: &'info mut Account<Token>,
    #[account(mut)]
    pub payer: &'info Signer,
    pub token_program: &'info Program<Token>,
    pub system_program: &'info Program<System>,
}

impl SwapExactTokensForTokens<'_> {
    #[inline(always)]
    pub fn swap_exact_tokens_for_tokens(
        &mut self,
        swap_a: bool,
        input_amount: u64,
        min_output_amount: u64,
        bumps: &SwapExactTokensForTokensBumps,
    ) -> Result<(), ProgramError> {
        // Clamp input to what the trader has.
        let input = if swap_a {
            let trader_a = self.trader_account_a.amount();
            if input_amount > trader_a { trader_a } else { input_amount }
        } else {
            let trader_b = self.trader_account_b.amount();
            if input_amount > trader_b { trader_b } else { input_amount }
        };

        // Apply fee.
        let fee = self.amm.fee.get() as u64;
        let taxed_input = input - input * fee / 10000;

        // Constant-product formula: output = taxed_input * pool_out / (pool_in + taxed_input)
        let pool_a = self.pool_account_a.amount();
        let pool_b = self.pool_account_b.amount();

        let output = if swap_a {
            (taxed_input as u128)
                .checked_mul(pool_b as u128)
                .ok_or(ProgramError::ArithmeticOverflow)?
                .checked_div(
                    (pool_a as u128)
                        .checked_add(taxed_input as u128)
                        .ok_or(ProgramError::ArithmeticOverflow)?,
                )
                .ok_or(ProgramError::ArithmeticOverflow)? as u64
        } else {
            (taxed_input as u128)
                .checked_mul(pool_a as u128)
                .ok_or(ProgramError::ArithmeticOverflow)?
                .checked_div(
                    (pool_b as u128)
                        .checked_add(taxed_input as u128)
                        .ok_or(ProgramError::ArithmeticOverflow)?,
                )
                .ok_or(ProgramError::ArithmeticOverflow)? as u64
        };

        if output < min_output_amount {
            return Err(ProgramError::Custom(4)); // OutputTooSmall
        }

        // Record invariant before the trade.
        let invariant = (pool_a as u128)
            .checked_mul(pool_b as u128)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Build authority signer seeds.
        let bump = [bumps.pool_authority];
        let seeds: &[Seed] = &[
            Seed::from(self.amm.address().as_ref()),
            Seed::from(self.mint_a.address().as_ref()),
            Seed::from(self.mint_b.address().as_ref()),
            Seed::from(crate::AUTHORITY_SEED),
            Seed::from(&bump as &[u8]),
        ];

        if swap_a {
            // Trader sends token A to pool.
            self.token_program
                .transfer(self.trader_account_a, self.pool_account_a, self.trader, input)
                .invoke()?;
            // Pool sends token B to trader (signed).
            self.token_program
                .transfer(self.pool_account_b, self.trader_account_b, self.pool_authority, output)
                .invoke_signed(seeds)?;
        } else {
            // Pool sends token A to trader (signed).
            self.token_program
                .transfer(self.pool_account_a, self.trader_account_a, self.pool_authority, output)
                .invoke_signed(seeds)?;
            // Trader sends token B to pool.
            self.token_program
                .transfer(self.trader_account_b, self.pool_account_b, self.trader, input)
                .invoke()?;
        }

        // Verify invariant holds (new product >= old product).
        let new_pool_a = pool_a as u128
            + if swap_a { input as u128 } else { 0 }
            - if !swap_a { output as u128 } else { 0 };
        let new_pool_b = pool_b as u128
            + if !swap_a { input as u128 } else { 0 }
            - if swap_a { output as u128 } else { 0 };
        let new_invariant = new_pool_a
            .checked_mul(new_pool_b)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        if new_invariant < invariant {
            return Err(ProgramError::Custom(5)); // InvariantViolated
        }

        Ok(())
    }
}
