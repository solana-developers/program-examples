#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
mod state;
#[cfg(test)]
mod tests;

declare_id!("E64FVeubGC4NPNF2UBJYX4AkrVowf74fRJD9q6YhwstN");

#[program]
mod quasar_lever {
    use super::*;

    /// Initialize the power status account (off by default).
    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<InitializeLever>) -> Result<(), ProgramError> {
        instructions::handle_initialize(&mut ctx.accounts)
    }

    /// Toggle the power switch. Logs who is pulling the lever.
    #[instruction(discriminator = 1)]
    pub fn switch_power(ctx: Ctx<SwitchPower>, name: String) -> Result<(), ProgramError> {
        instructions::handle_switch_power(&mut ctx.accounts, name)
    }
}
