#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
#[cfg(test)]
mod tests;

declare_id!("ED6f4gweAE7hWPQPXMt4kWxzDJne8VQEm9zkb1tMpFNB");

#[program]
mod quasar_rent {
    use super::*;

    /// Create a system account with enough lamports for rent exemption,
    /// sized to hold the given address data (name + address strings).
    ///
    /// The Anchor version takes an `AddressData` struct, but Quasar doesn't
    /// yet support custom struct instruction args in client codegen
    /// (blueshift-gg/quasar#126). We pass the fields individually instead.
    #[instruction(discriminator = 0)]
    pub fn create_system_account(
        ctx: Ctx<CreateSystemAccount>,
        name: String,
        address: String,
    ) -> Result<(), ProgramError> {
        ctx.accounts.create_system_account(name, address)
    }
}
