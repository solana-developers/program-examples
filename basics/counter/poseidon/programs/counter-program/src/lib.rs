use anchor_lang::prelude::*;
declare_id!("DMATyR7jooijeJ2aJYWiyYPf3eoUouumaaLw1JbG3TYF");
#[program]
pub mod counter_program {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        ctx.accounts.state.count = 0;
        Ok(())
    }
    pub fn increment(ctx: Context<IncrementContext>) -> Result<()> {
        ctx.accounts.state.count = ctx.accounts.state.count + 1;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 17, seeds = [b"count"], bump)]
    pub state: Account<'info, CounterState>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct IncrementContext<'info> {
    #[account(mut, seeds = [b"count"], bump)]
    pub state: Account<'info, CounterState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct CounterState {
    pub count: u64,
    pub bump: u8,
}
