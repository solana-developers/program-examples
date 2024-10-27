use anchor_lang::prelude::*;
declare_id!("AAfRjjKbh77KLxouxEymo5uzJ4qbRaqorx5gHVuX59o8");
#[program]
pub mod hello_solana {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        ctx.accounts.counter.authority = ctx.accounts.authority.key();
        Ok(())
    }
    pub fn increment(ctx: Context<IncrementContext>) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(
        init,
        payer = authority,
        space = 41,
        seeds = [b"counter",
        authority.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct IncrementContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account()]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub value: u8,
}
