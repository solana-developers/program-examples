use anchor_lang::prelude::*;
declare_id!("36PvqMz57YD68SfLBTLL9bYhQdw1BdL4kGKK3krdVoSA");
#[program]
pub mod counter_program {
    use super::*;
    pub fn initialize_counter(ctx: Context<InitializeCounterContext>) -> Result<()> {
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
pub struct InitializeCounterContext<'info> {
    #[account(init, payer = user, space = 17, seeds = [b"count"], bump)]
    pub state: Account<'info, CounterState>,
    #[account(mut)]
    pub user: Signer<'info>,
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
    pub count: i64,
    pub bump: u8,
}
