use anchor_lang::prelude::*;

declare_id!("BmDHboaj1kBUoinJKKSRqKfMeRKJqQqEbUj1VgzeQe4A");

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
    #[account(init, payer = payer, space = 17, seeds = [b"counter"], bump)]
    pub state: Account<'info, CounterState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IncrementContext<'info> {
    #[account(mut, seeds = [b"counter"], bump)]
    pub state: Account<'info, CounterState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DecrementContext<'info> {
    #[account(mut, seeds = [b"counter"], bump)]
    pub state: Account<'info, CounterState>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CounterState {
    pub count: u64,
    pub bump: u8,
}
