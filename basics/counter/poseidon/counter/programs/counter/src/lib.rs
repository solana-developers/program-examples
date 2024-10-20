use anchor_lang::prelude::*;
declare_id!("Hn5fB7seeqBPGWWXQCnoA4bomJy9H8ktX9f2HiGtGWP1");
#[program]
pub mod counter {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        ctx.accounts.state.count = 0;
        Ok(())
    }
    pub fn increment(ctx: Context<IncrementContext>) -> Result<()> {
        ctx.accounts.state.count = ctx.accounts.state.count + 1;
        Ok(())
    }
    pub fn decrement(ctx: Context<DecrementContext>) -> Result<()> {
        ctx.accounts.state.count = ctx.accounts.state.count - 1;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(init, payer = payer, space = 16, seeds = [b"count"], bump)]
    pub state: Account<'info, CounterState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct IncrementContext<'info> {
    #[account(mut, seeds = [b"count"], bump)]
    pub state: Account<'info, CounterState>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct DecrementContext<'info> {
    #[account(mut, seeds = [b"count"], bump)]
    pub state: Account<'info, CounterState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct CounterState {
    pub count: u64,
}
