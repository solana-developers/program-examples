use anchor_lang::prelude::*;
declare_id!("7yvcNv9BAHHZYPgDag1YFSLEbXiwBTmVmuE4eArSSEKH");
#[program]
pub mod counter_program {
    use super::*;
    pub fn initialize_counter(ctx: Context<InitializeCounterContext>) -> Result<()> {
        ctx.accounts.counter.count = 0;
        Ok(())
    }
    pub fn increment(ctx: Context<IncrementContext>) -> Result<()> {
        ctx.accounts.counter.count = ctx.accounts.counter.count + 1;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeCounterContext<'info> {
    #[account(mut, seeds = [b"count"], bump)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct IncrementContext<'info> {
    #[account(mut, seeds = [b"count"], bump)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Counter {
    pub count: u64,
    pub bump: u8,
}
