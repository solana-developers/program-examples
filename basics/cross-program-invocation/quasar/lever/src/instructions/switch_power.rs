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

impl<'info> SwitchPower<'info> {
    #[inline(always)]
    pub fn switch_power(&mut self, _name: &str) -> Result<(), ProgramError> {
        let current: bool = self.power.is_on.into();
        let new_state = !current;
        self.power.is_on = PodBool::from(new_state);

        // Quasar's log() takes &str — no format! in no_std.
        log("Someone is pulling the power switch!");

        if new_state {
            log("The power is now on.");
        } else {
            log("The power is now off!");
        }

        Ok(())
    }
}
