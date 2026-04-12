use anchor_lang::prelude::*;

declare_id!("Bi5N7SUQhpGknVcqPTzdFFVueQoxoUu8YTLz75J6fT8A");

// automatically generate module using program idl found in ./idls
declare_program!(lever);
use lever::accounts::PowerStatus;
use lever::cpi::accounts::SwitchPower;
use lever::cpi::switch_power;
use lever::program::Lever;

#[program]
pub mod hand {
    use super::*;

    pub fn pull_lever(context: Context<PullLeverAccountConstraints>, name: String) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            context.accounts.lever_program.key(),
            SwitchPower {
                power: context.accounts.power.to_account_info(),
            },
        );
        switch_power(cpi_ctx, name)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PullLeverAccountConstraints<'info> {
    #[account(mut)]
    pub power: Account<'info, PowerStatus>,
    pub lever_program: Program<'info, Lever>,
}
