#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
#[cfg(test)]
mod tests;

declare_id!("ECWPhR3rJbaPfyNFgphnjxSEexbTArc7vxD8fnW6tgKw");

#[program]
mod quasar_checking_accounts {
    use super::*;

    /// Account validation in Quasar is done using the types in #[derive(Accounts)] structs:
    /// - Signer: checks the account has signed the transaction
    /// - UncheckedAccount: no validation (opt-in to unchecked access)
    /// - Program<System>: checks account is executable and is the system program
    #[instruction(discriminator = 0)]
    pub fn check_accounts(ctx: Ctx<CheckAccounts>) -> Result<(), ProgramError> {
        ctx.accounts.check_accounts()
    }
}
