use anchor_lang::prelude::*;
declare_id!("D4aA71us8bTcdXeZQpXyXidW2xPugVwUuoXx3b1bnvXa");
#[program]
pub mod cross_program_invocation {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        Ok(())
    }
    pub fn switch_power(ctx: Context<SwitchPowerContext>, name: String) -> Result<()> {
        Ok(())
    }
    pub fn pull_lever(ctx: Context<PullLeverContext>, name: String) -> Result<()> {
        Ok(())
    }
    pub fn initialize_lever(ctx: Context<InitializeLeverContext>) -> Result<()> {
        Ok(())
    }
    pub fn set_power_status(ctx: Context<SetPowerStatusContext>) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {}
#[derive(Accounts)]
pub struct SwitchPowerContext<'info> {}
#[derive(Accounts)]
pub struct PullLeverContext<'info> {
    #[account()]
    pub lever_program: Account<'info, Lever>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account()]
    pub power: Account<'info, PowerStatus>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct InitializeLeverContext<'info> {
    #[account(init, payer = user, space = 8, seeds = [b"power"], bump)]
    pub power: Account<'info, PowerStatus>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SetPowerStatusContext<'info> {
    #[account()]
    pub power: Account<'info, PowerStatus>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct PowerStatus {}
#[account]
pub struct Lever {}
