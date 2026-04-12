#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
mod state;
#[cfg(test)]
mod tests;

declare_id!("Fod47xKXjdHVQDzkFPBvfdWLm8gEAV4iMSXkfUzCHiSD");

#[program]
mod quasar_realloc {
    use super::*;

    /// Create a message account with an initial message.
    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<Initialize>, message: String) -> Result<(), ProgramError> {
        ctx.accounts.initialize(message)
    }

    /// Update the message, reallocating if the new message is longer.
    /// Quasar's `set_inner` handles realloc transparently.
    #[instruction(discriminator = 1)]
    pub fn update(ctx: Ctx<Update>, message: String) -> Result<(), ProgramError> {
        ctx.accounts.update(message)
    }
}
