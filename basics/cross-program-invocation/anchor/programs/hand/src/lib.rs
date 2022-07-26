use anchor_lang::prelude::*;
use lever::cpi::accounts::SetPowerStatus;
use lever::program::Lever;
use lever::{self, PowerStatus};


declare_id!("ABoYG2GWbzLgnnGhK2pUGNupzKoYe7UGk2idrAXbstAS");


#[program]
mod hand {
    use super::*;
    pub fn pull_lever(ctx: Context<PullLever>, name: String) -> anchor_lang::Result<()> {
        let cpi_program = ctx.accounts.lever_program.to_account_info();
        let cpi_accounts = SetPowerStatus {
            power: ctx.accounts.power.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        lever::cpi::switch_power(cpi_ctx, name)
    }
}


#[derive(Accounts)]
pub struct PullLever<'info> {
    #[account(mut)]
    pub power: Account<'info, PowerStatus>,
    pub lever_program: Program<'info, Lever>,
}