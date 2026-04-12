#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
#[cfg(test)]
mod tests;

declare_id!("DgoL5J44aspizyUs9fcnpGEUJjWTLJRCfx8eYtUMYczf");

#[program]
mod quasar_processing_instructions {
    use super::*;

    /// Process instruction data: name (String) and height (u32).
    /// Quasar can parse String instruction args (u32-prefixed wire format) but
    /// can't interpolate them into log messages (no format! in no_std).
    #[instruction(discriminator = 0)]
    pub fn go_to_park(ctx: Ctx<Park>, name: String, height: u32) -> Result<(), ProgramError> {
        instructions::handle_go_to_park(&mut ctx.accounts, name, height)
    }
}
