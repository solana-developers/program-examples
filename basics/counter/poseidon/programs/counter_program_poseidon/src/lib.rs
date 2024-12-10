use anchor_lang::prelude::*;
declare_id!("E2BQKu6JgAjXowgA463L2rBSg9LDh1qysV1qH9BxRXPn");
#[program]
pub mod counter_program_poseidon {
    use super::*;
    pub fn initialize_counter(ctx: Context<InitializeCounterContext>) -> Result<()> {
        ctx.accounts.counter.count = 0;
        Ok(())
    }
    pub fn increment_counter(ctx: Context<IncrementCounterContext>) -> Result<()> {
        ctx.accounts.counter.count = ctx.accounts.counter.count + 1;
        Ok(())
    }
    pub fn decrement_counter(ctx: Context<DecrementCounterContext>) -> Result<()> {
        ctx.accounts.counter.count = ctx.accounts.counter.count - 1;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeCounterContext<'info> {
    #[account(init, payer = user, space = 8 + CounterState::INIT_SPACE, seeds = [b"count"], bump)]
    pub counter: Account<'info, CounterState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct IncrementCounterContext<'info> {
    #[account(mut, seeds = [b"count"], bump)]
    pub counter: Account<'info, CounterState>,
    pub system_program: Program<'info, System>,
  
}
#[derive(Accounts)]
pub struct DecrementCounterContext<'info> {
    #[account(mut, seeds = [b"count"], bump)]
    pub counter: Account<'info, CounterState>,
    pub system_program: Program<'info, System>,
   
}
#[account]
#[derive(InitSpace)]
pub struct CounterState {
    pub count: i64,
    pub bump: u8,
}
