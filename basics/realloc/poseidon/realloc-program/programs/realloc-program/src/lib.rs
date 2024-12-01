use anchor_lang::prelude::*;
declare_id!("7T1DgawXjJD6kGaC43ujSw2xXLhn7w28MGzyD7oV8Q1B");
#[program]
pub mod realloc_program {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>, input: String) -> Result<()> {
        ctx.accounts.account.message = input;
        ctx.accounts.account.bump = ctx.bumps.account;
        Ok(())
    }
    pub fn update(ctx: Context<UpdateContext>, input: String) -> Result<()> {
        ctx.accounts.account.message = input;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, space = 32, seeds = [b"message"], bump)]
    pub account: Account<'info, MessageAccountState>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct UpdateContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    // Due to current limitations in Poseidon, dynamic allocation (reallocation) is not supported on Poseidon right now.
    // As a result, this example uses fixed-sized fields to work around the limitation.
    // In typical Solana programs using Anchor, dynamic reallocation allows accounts to resize based on the input data.
    // so I am adding it manually right now
    #[account(
        mut,
        realloc = 48,
        realloc::payer = payer,
        realloc::zero = true,
    )]
    pub account: Account<'info, MessageAccountState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct MessageAccountState {
    pub message: String,
    pub bump: u8,
}
