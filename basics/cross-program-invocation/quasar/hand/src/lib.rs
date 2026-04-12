#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
#[cfg(test)]
mod tests;

declare_id!("Bi5N7SUQhpGknVcqPTzdFFVueQoxoUu8YTLz75J6fT8A");

/// The lever program's ID — used to verify the correct program is passed.
pub const LEVER_PROGRAM_ID: Address = address!("E64FVeubGC4NPNF2UBJYX4AkrVowf74fRJD9q6YhwstN");

/// Marker type for the lever program, implementing `Id` so it can be used
/// with `Program<LeverProgram>` in the accounts struct.
pub struct LeverProgram;

impl Id for LeverProgram {
    const ID: Address = LEVER_PROGRAM_ID;
}

#[program]
mod quasar_hand {
    use super::*;

    /// Pull the lever by invoking the lever program's switch_power via CPI.
    #[instruction(discriminator = 0)]
    pub fn pull_lever(ctx: Ctx<PullLever>, name: String) -> Result<(), ProgramError> {
        instructions::handle_pull_lever(&mut ctx.accounts, name)
    }
}
