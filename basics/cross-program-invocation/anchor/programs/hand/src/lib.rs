#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use lever::cpi::accounts::SetPowerStatus;
use lever::program::Lever;
use lever::{self, PowerStatus};

declare_id!("ABoYG2GWbzLgnnGhK2pUGNupzKoYe7UGk2idrAXbstAS");

#[program]
mod hand {
    use super::*;
    pub fn pull_lever(ctx: Context<PullLever>, name: String) -> anchor_lang::Result<()> {
        // Hitting the switch_power method on the lever program
        //
        lever::cpi::switch_power(
            CpiContext::new(
                ctx.accounts.lever_program.to_account_info(),
                // Using the accounts context struct from the lever program
                //
                SetPowerStatus {
                    power: ctx.accounts.power.to_account_info(),
                },
            ),
            name,
        )
    }
}

#[derive(Accounts)]
pub struct PullLever<'info> {
    #[account(mut)]
    pub power: Account<'info, PowerStatus>,
    pub lever_program: Program<'info, Lever>,
}
