use anchor_lang::prelude::*;
declare_id!("Cd86dtBUzQKYTFtcB8zDxPRUPCtKPocyetWZSnq6PNxv");
#[program]
pub mod hand {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        Ok(())
    }
    pub fn pull_lever(ctx: Context<PullLeverContext>) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub power: Signer<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}
#[derive(Accounts)]
pub struct PullLeverContext<'info> {
    #[account(init, payer = user, space = 8, seeds = [b"hand"], bump)]
    pub power: Account<'info, PowerStatus>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct PowerStatus {}
