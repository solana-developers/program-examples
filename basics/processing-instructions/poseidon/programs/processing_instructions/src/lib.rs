use anchor_lang::prelude::*;
declare_id!("85pSnBtvLk9JEwKDuQUH3AWx7vdt6bndneWi9fyrBkhF");
#[program]
pub mod processing_instructions {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>, time: u64) -> Result<()> {
        ctx.accounts.greeting.last_updated = time;
        Ok(())
    }
    pub fn update_greeting(
        ctx: Context<UpdateGreetingContext>,
        time: u64,
    ) -> Result<()> {
        ctx.accounts.greeting.last_updated = time;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, space = 16, seeds = [b"greeting"], bump)]
    pub greeting: Account<'info, GreetingAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct UpdateGreetingContext<'info> {
    #[account(mut)]
    pub greeting: Account<'info, GreetingAccount>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct GreetingAccount {
    pub last_updated: u64,
}
