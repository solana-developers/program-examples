use anchor_lang::prelude::*;
declare_id!("GnL9WWgvnFbhvNedx6LTdPt4QeWXM4XdAtnRE4uToXdV");
#[program]
pub mod counter {
    use super::*;
    pub fn initialize_counter(ctx: Context<InitializeCounterContext>) -> Result<()> {
        ctx.accounts.counter.count = 0;
        Ok(())
    }
    pub fn increment_counter(ctx: Context<IncrementCounterContext>) -> Result<()> {
        ctx.accounts.counter.count = ctx.accounts.counter.count + 1;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeCounterContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, space = 16, seeds = [b"counter"], bump)]
    pub counter: Account<'info, CounterAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct IncrementCounterContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"counter"], bump)]
    pub counter: Account<'info, CounterAccount>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct CounterAccount {
    pub count: u64,
}
