use anchor_lang::prelude::*;
declare_id!("3dhKkikKk112wEVdNr69Q2eEXSwU3MivfTNxauQsTjJP");
#[program]
pub mod counter {
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
    #[account(
        init,
        payer = authority,
        space = 17,
        seeds = [b"counter",
        authority.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, CounterAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct IncrementContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"counter", authority.key().as_ref()], bump = counter.bump)]
    pub counter: Account<'info, CounterAccount>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct CounterAccount {
    pub count: u64,
    pub bump: u8,
}
