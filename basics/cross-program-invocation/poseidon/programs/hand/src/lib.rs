use anchor_lang::prelude::*;
declare_id!("Cd86dtBUzQKYTFtcB8zDxPRUPCtKPocyetWZSnq6PNxv");
#[program]
pub mod hand {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        Ok(())
    }
    pub fn pull_lever(ctx: Context<PullLeverContext>, name: String) -> Result<()> {
        Ok(())
    }
    pub fn switch_power(ctx: Context<SwitchPowerContext>, name: String) -> Result<()> {
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
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 9, seeds = [b"hand"], bump)]
    pub power: Account<'info, PowerStatus>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SwitchPowerContext {}
#[account]
pub struct PowerStatus {
    pub is_on: bool,
}
