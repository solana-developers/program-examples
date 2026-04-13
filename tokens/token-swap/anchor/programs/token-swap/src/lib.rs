use anchor_lang::prelude::*;

mod constants;
mod errors;
mod instructions;
mod state;

// Set the correct key here
declare_id!("QmzKmhyUQ9jbNKCPWQjqYNcrqek3FVjSSxc4sCtJeJL");

#[program]
pub mod swap_example {
    pub use super::instructions::*;
    use super::*;

    pub fn create_amm(context: Context<CreateAmmAccountConstraints>, id: Pubkey, fee: u16) -> Result<()> {
        instructions::handle_create_amm(context, id, fee)
    }

    pub fn create_pool(context: Context<CreatePoolAccountConstraints>) -> Result<()> {
        instructions::handle_create_pool(context)
    }

    pub fn deposit_liquidity(
        context: Context<DepositLiquidityAccountConstraints>,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        instructions::handle_deposit_liquidity(context, amount_a, amount_b)
    }

    pub fn withdraw_liquidity(context: Context<WithdrawLiquidityAccountConstraints>, amount: u64) -> Result<()> {
        instructions::handle_withdraw_liquidity(context, amount)
    }

    pub fn swap_exact_tokens_for_tokens(
        context: Context<SwapExactTokensForTokensAccountConstraints>,
        swap_a: bool,
        input_amount: u64,
        min_output_amount: u64,
    ) -> Result<()> {
        instructions::handle_swap_exact_tokens_for_tokens(context, swap_a, input_amount, min_output_amount)
    }
}
