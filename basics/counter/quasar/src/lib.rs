#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
mod state;
#[cfg(test)]
mod tests;

declare_id!("HYSDBQLVUSMRQKQZxfKJwDy5PPrZb7bvuBLaWfbcYhEP");

#[program]
mod quasar_counter {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn initialize_counter(ctx: Ctx<InitializeCounter>) -> Result<(), ProgramError> {
        ctx.accounts.initialize_counter()
    }

    #[instruction(discriminator = 1)]
    pub fn increment(ctx: Ctx<Increment>) -> Result<(), ProgramError> {
        ctx.accounts.increment()
    }
}
