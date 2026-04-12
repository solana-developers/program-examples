#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
mod state;
#[cfg(test)]
mod tests;

declare_id!("99TQtoDdQ5NS2v5Ppha93aqEmv3vV9VZVfHTP5rGST3c");

#[program]
mod quasar_close_account {
    use super::*;

    /// Create a user account with a name.
    #[instruction(discriminator = 0)]
    pub fn create_user(ctx: Ctx<CreateUser>, name: String) -> Result<(), ProgramError> {
        let bump = ctx.bumps.user_account;
        ctx.accounts.create_user(name, bump)
    }

    /// Close a user account and return lamports to the user.
    #[instruction(discriminator = 1)]
    pub fn close_user(ctx: Ctx<CloseUser>) -> Result<(), ProgramError> {
        ctx.accounts.close_user()
    }
}
