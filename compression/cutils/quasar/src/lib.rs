#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod bubblegum_types;
mod instructions;
mod state;
use instructions::*;
#[cfg(test)]
mod tests;

/// SPL Account Compression program ID (cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK).
const SPL_ACCOUNT_COMPRESSION_ID: Address = Address::new_from_array([
    0x09, 0x2a, 0x13, 0xee, 0x95, 0xc4, 0x1c, 0xba, 0x08, 0xa6, 0x7f, 0x5a, 0xc6, 0x7e, 0x8d,
    0xf7, 0xe1, 0xda, 0x11, 0x62, 0x5e, 0x1d, 0x64, 0x13, 0x7f, 0x8f, 0x4f, 0x23, 0x83, 0x03,
    0x7f, 0x14,
]);

/// mpl-bubblegum program ID (BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY).
const MPL_BUBBLEGUM_ID: Address = Address::new_from_array([
    0x98, 0x8b, 0x80, 0xeb, 0x79, 0x35, 0x28, 0x69, 0xb2, 0x24, 0x74, 0x5f, 0x59, 0xdd, 0xbf,
    0x8a, 0x26, 0x58, 0xca, 0x13, 0xdc, 0x68, 0x81, 0x21, 0x26, 0x35, 0x1c, 0xae, 0x07, 0xc1,
    0xa5, 0xa5,
]);

declare_id!("BuFyrgRYzg2nPhqYrxZ7d9uYUs4VXtxH71U8EcoAfTQZ");

#[program]
mod quasar_cutils {
    use super::*;

    /// Mint a compressed NFT to a collection via MintToCollectionV1.
    #[instruction(discriminator = 0)]
    pub fn mint(ctx: Ctx<Mint>) -> Result<(), ProgramError> {
        instructions::handle_mint(&ctx.accounts, &ctx)
    }

    /// Verify a compressed NFT leaf exists in the merkle tree.
    #[instruction(discriminator = 1)]
    pub fn verify(ctx: CtxWithRemaining<Verify>) -> Result<(), ProgramError> {
        instructions::handle_verify(&ctx.accounts, &ctx)
    }
}
