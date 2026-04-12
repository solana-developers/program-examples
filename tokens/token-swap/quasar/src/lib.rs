#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
pub mod state;
#[cfg(test)]
mod tests;

declare_id!("22222222222222222222222222222222222222222222");

/// Minimum liquidity locked on first deposit to prevent manipulation.
pub const MINIMUM_LIQUIDITY: u64 = 100;
/// Seed for the pool authority PDA.
pub const AUTHORITY_SEED: &[u8] = b"authority";
/// Seed for the liquidity mint PDA.
pub const LIQUIDITY_SEED: &[u8] = b"liquidity";

/// Simple constant-product AMM (token swap).
///
/// Five instructions:
/// 1. `create_amm` — register a new AMM with admin + fee
/// 2. `create_pool` — create a liquidity pool for a token pair
/// 3. `deposit_liquidity` — add liquidity and receive LP tokens
/// 4. `withdraw_liquidity` — burn LP tokens and receive pool tokens
/// 5. `swap_exact_tokens_for_tokens` — swap one token for another
#[program]
mod quasar_token_swap {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn create_amm(
        ctx: Ctx<CreateAmm>,
        id: Address,
        fee: u16,
    ) -> Result<(), ProgramError> {
        ctx.accounts.create_amm(id, fee)
    }

    #[instruction(discriminator = 1)]
    pub fn create_pool(ctx: Ctx<CreatePool>) -> Result<(), ProgramError> {
        ctx.accounts.create_pool()
    }

    #[instruction(discriminator = 2)]
    pub fn deposit_liquidity(
        ctx: Ctx<DepositLiquidity>,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<(), ProgramError> {
        ctx.accounts.deposit_liquidity(amount_a, amount_b, &ctx.bumps)
    }

    #[instruction(discriminator = 3)]
    pub fn withdraw_liquidity(
        ctx: Ctx<WithdrawLiquidity>,
        amount: u64,
    ) -> Result<(), ProgramError> {
        ctx.accounts.withdraw_liquidity(amount, &ctx.bumps)
    }

    #[instruction(discriminator = 4)]
    pub fn swap_exact_tokens_for_tokens(
        ctx: Ctx<SwapExactTokensForTokens>,
        swap_a: bool,
        input_amount: u64,
        min_output_amount: u64,
    ) -> Result<(), ProgramError> {
        ctx.accounts
            .swap_exact_tokens_for_tokens(swap_a, input_amount, min_output_amount, &ctx.bumps)
    }
}
