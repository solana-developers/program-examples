use anchor_lang::prelude::*;

declare_id!("E64FVeubGC4NPNF2UBJYX4AkrVowf74fRJD9q6YhwstN");

#[program]
pub mod lever {
    use super::*;

    pub fn initialize(_ctx: Context<InitializeLever>) -> Result<()> {
        Ok(())
    }

    pub fn switch_power(ctx: Context<SetPowerStatus>, name: String) -> Result<()> {
        let power = &mut ctx.accounts.power;
        power.is_on = !power.is_on;

        msg!("{} is pulling the power switch!", &name);

        match power.is_on {
            true => msg!("The power is now on."),
            false => msg!("The power is now off!"),
        };

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeLever<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub power: Account<'info, PowerStatus>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetPowerStatus<'info> {
    #[account(mut)]
    pub power: Account<'info, PowerStatus>,
}

#[account]
pub struct PowerStatus {
    pub is_on: bool,
}
