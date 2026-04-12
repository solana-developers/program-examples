#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
mod state;
#[cfg(test)]
mod tests;

declare_id!("22222222222222222222222222222222222222222222");

/// Token escrow program: a maker deposits token A into a vault and specifies
/// how much of token B they want in return. A taker fulfils the offer by
/// sending the requested token B and receiving the deposited token A.
#[program]
mod quasar_escrow {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn make(ctx: Ctx<Make>, deposit: u64, receive: u64) -> Result<(), ProgramError> {
        ctx.accounts.make_escrow(receive, &ctx.bumps)?;
        ctx.accounts.deposit_tokens(deposit)
    }

    #[instruction(discriminator = 1)]
    pub fn take(ctx: Ctx<Take>) -> Result<(), ProgramError> {
        ctx.accounts.transfer_tokens()?;
        ctx.accounts.withdraw_tokens_and_close(&ctx.bumps)
    }

    #[instruction(discriminator = 2)]
    pub fn refund(ctx: Ctx<Refund>) -> Result<(), ProgramError> {
        ctx.accounts.withdraw_tokens_and_close(&ctx.bumps)
    }
}
