#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
#[cfg(test)]
mod tests;

declare_id!("7Hm9nsYVuBZ9rf8z9AMUHreZRv8Q4vLhqwdVTCawRZtA");

#[program]
mod quasar_pda_rent_payer {
    use super::*;

    /// Fund a PDA "rent vault" by transferring lamports from the payer.
    #[instruction(discriminator = 0)]
    pub fn init_rent_vault(ctx: Ctx<InitRentVault>, fund_lamports: u64) -> Result<(), ProgramError> {
        instructions::handle_init_rent_vault(&mut ctx.accounts, fund_lamports)
    }

    /// Create a new account using the rent vault PDA as the funding source.
    /// The vault signs the CPI via PDA seeds.
    #[instruction(discriminator = 1)]
    pub fn create_new_account(ctx: Ctx<CreateNewAccount>) -> Result<(), ProgramError> {
        instructions::handle_create_new_account(&mut ctx.accounts, ctx.bumps.rent_vault)
    }
}
