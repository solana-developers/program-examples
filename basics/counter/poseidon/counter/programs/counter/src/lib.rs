use anchor_lang::prelude::*;
declare_id!("FceHwEvpsYuAawfi4Lcp5LRtb6Hze97YrgGpQHUXwNwo");
#[program]
pub mod counter_program {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        ctx.accounts.counter.count = 0;
        ctx.accounts.counter.bump = ctx.bumps.counter;
        Ok(())
    }
    pub fn increment(ctx: Context<IncrementContext>) -> Result<()> {
        ctx.accounts.counter.count = ctx.accounts.counter.count + 1;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 17,
        seeds = [b"counter",
        authority.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct IncrementContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"counter", authority.key().as_ref()], bump = counter.bump)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Counter {
    pub count: u64,
    pub bump: u8,
}
