#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
#[cfg(test)]
mod tests;

declare_id!("GUkjQmrLPFXXNK1bFLKt8XQi6g3TjxcHVspbjDoHvMG2");

#[program]
mod quasar_pyth_example {
    use super::*;

    /// Read and log Pyth price feed data from a PriceUpdateV2 account.
    #[instruction(discriminator = 0)]
    pub fn read_price(ctx: Ctx<ReadPrice>) -> Result<(), ProgramError> {
        instructions::handle_read_price(&mut ctx.accounts)
    }
}
