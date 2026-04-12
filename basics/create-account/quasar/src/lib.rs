#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
#[cfg(test)]
mod tests;

declare_id!("ARVNCsYKDQsCLHbwUTJLpFXVrJdjhWZStyzvxmKe2xHi");

#[program]
mod quasar_create_account {
    use super::*;

    /// Create a new system-owned account via CPI to the system program.
    #[instruction(discriminator = 0)]
    pub fn create_system_account(ctx: Ctx<CreateSystemAccount>) -> Result<(), ProgramError> {
        ctx.accounts.create_system_account()
    }
}
