use {
    crate::state::PowerStatus,
    quasar_lang::prelude::*,
};

/// Accounts for toggling the power switch.
#[derive(Accounts)]
pub struct SwitchPower<'info> {
    #[account(mut)]
    pub power: &'info mut Account<PowerStatus>,
}

#[inline(always)]
pub fn handle_switch_power(accounts: &mut SwitchPower, _name: &str) -> Result<(), ProgramError> {
    let current: bool = accounts.power.is_on.into();
    let new_state = !current;
    accounts.power.is_on = PodBool::from(new_state);

    // Quasar's log() takes &str — no format! in no_std.
    log("Someone is pulling the power switch!");

    if new_state {
        log("The power is now on.");
    } else {
        log("The power is now off!");
    }

    Ok(())
}
