use {
    crate::state::PowerStatus,
    quasar_lang::prelude::*,
};

/// Accounts for initialising the power status (PDA seeded by "power").
#[derive(Accounts)]
pub struct InitializeLever<'info> {
    #[account(mut)]
    pub payer: &'info mut Signer,
    #[account(mut, init, payer = payer, seeds = [b"power"], bump)]
    pub power: &'info mut Account<PowerStatus>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_initialize(accounts: &mut InitializeLever) -> Result<(), ProgramError> {
    // Power starts off (false).
    accounts.power.set_inner(PodBool::from(false));
    Ok(())
}
