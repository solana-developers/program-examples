#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
#[cfg(test)]
mod tests;

declare_id!("G4eCqMUNnR2q7Ej9Ep2rURUM4gXdZ7RswqU9QPjgSGrz");

#[program]
mod quasar_transfer_sol {
    use super::*;

    /// Transfer SOL from payer to recipient via system program CPI.
    #[instruction(discriminator = 0)]
    pub fn transfer_sol_with_cpi(
        ctx: Ctx<TransferSolWithCpi>,
        amount: u64,
    ) -> Result<(), ProgramError> {
        instructions::handle_transfer_sol_with_cpi(&mut ctx.accounts, amount)
    }

    /// Transfer SOL by directly manipulating lamports.
    /// The payer account must be owned by this program.
    #[instruction(discriminator = 1)]
    pub fn transfer_sol_with_program(
        ctx: Ctx<TransferSolWithProgram>,
        amount: u64,
    ) -> Result<(), ProgramError> {
        instructions::handle_transfer_sol_with_program(&mut ctx.accounts, amount)
    }
}
