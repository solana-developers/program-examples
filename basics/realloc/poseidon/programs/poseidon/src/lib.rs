use anchor_lang::prelude::*;
declare_id!("2TVLNyk3jZCVNQ5UVJQRFPdjY3APCToU77isidjB3re4");
#[program]
pub mod realloc_example {
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
    #[account(init, payer = payer, space = 9, seeds = [b"message"], bump)]
    pub account: Account<'info, MessageAccountState>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct UpdateContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"message"], bump)]
    pub account: Account<'info, MessageAccountState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct MessageAccountState {
    pub message: String,
    pub bump: u8,
}
