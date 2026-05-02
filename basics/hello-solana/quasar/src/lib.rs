#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
#[cfg(test)]
mod tests;

declare_id!("FLUH9c5oAfXb1eYbkZvdGK9r9SLQJBUi2DZQaBVj7Tzr");

#[program]
mod quasar_hello_solana {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn hello(ctx: Ctx<Hello>) -> Result<(), ProgramError> {
        instructions::handle_hello(&mut ctx.accounts)
    }
}
