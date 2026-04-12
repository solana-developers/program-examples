#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
mod state;
#[cfg(test)]
mod tests;

declare_id!("oCCQRZyAbVxujyd8m57MPmDzZDmy2FoKW4ULS7KofCE");

#[program]
mod quasar_program_derived_addresses {
    use super::*;

    /// Create a PDA-based page visits counter for the payer.
    #[instruction(discriminator = 0)]
    pub fn create_page_visits(ctx: Ctx<CreatePageVisits>) -> Result<(), ProgramError> {
        ctx.accounts.create_page_visits()
    }

    /// Increment the page visits counter.
    #[instruction(discriminator = 1)]
    pub fn increment_page_visits(ctx: Ctx<IncrementPageVisits>) -> Result<(), ProgramError> {
        ctx.accounts.increment_page_visits()
    }
}
