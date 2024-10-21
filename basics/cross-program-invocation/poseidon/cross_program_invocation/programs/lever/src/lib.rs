use anchor_lang::prelude::*;
declare_id!("9aM9w7ozrZwXx9bQHbBx6QjWc6F46tdN9ayt86vt9uLL");
#[program]
pub mod lever {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
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
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub power: Signer<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}
#[derive(Accounts)]
pub struct InitializeLeverContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 8, seeds = [b"lever"], bump)]
    pub power: Account<'info, PowerStatus>,
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
