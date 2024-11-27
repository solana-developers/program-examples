use anchor_lang::prelude::*;
declare_id!("EgcUM7mn2dsedh9vjY8ihfzuU9Vhhady8bSPcRssUriR");
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
    pub fn decrement(ctx: Context<DecrementContext>) -> Result<()> {
        ctx.accounts.counter.count = ctx.accounts.counter.count - 1;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeCounterContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 49,
        seeds = [b"count",
        payer.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct IncrementContext<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct DecrementContext<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Counter {
    pub payer: Pubkey,
    pub count: u64,
    pub bump: u8,
}
