#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
#[cfg(test)]
mod tests;

/// mpl-bubblegum Transfer instruction discriminator.
const TRANSFER_DISCRIMINATOR: [u8; 8] = [163, 52, 200, 231, 140, 3, 69, 186];

/// mpl-bubblegum program ID (BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY).
const MPL_BUBBLEGUM_ID: Address = Address::new_from_array([
    0x98, 0x8b, 0x80, 0xeb, 0x79, 0x35, 0x28, 0x69, 0xb2, 0x24, 0x74, 0x5f, 0x59, 0xdd, 0xbf,
    0x8a, 0x26, 0x58, 0xca, 0x13, 0xdc, 0x68, 0x81, 0x21, 0x26, 0x35, 0x1c, 0xae, 0x07, 0xc1,
    0xa5, 0xa5,
]);

/// SPL Account Compression program ID (cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK).
const SPL_ACCOUNT_COMPRESSION_ID: Address = Address::new_from_array([
    0x09, 0x2a, 0x13, 0xee, 0x95, 0xc4, 0x1c, 0xba, 0x08, 0xa6, 0x7f, 0x5a, 0xc6, 0x7e, 0x8d,
    0xf7, 0xe1, 0xda, 0x11, 0x62, 0x5e, 0x1d, 0x64, 0x13, 0x7f, 0x8f, 0x4f, 0x23, 0x83, 0x03,
    0x7f, 0x14,
]);

declare_id!("Fd4iwpPWaCU8BNwGQGtvvrcvG4Tfizq3RgLm8YLBJX6D");

#[program]
mod quasar_cnft_vault {
    use super::*;

    /// Withdraw a single compressed NFT from the vault PDA.
    #[instruction(discriminator = 0)]
    pub fn withdraw_cnft(ctx: CtxWithRemaining<Withdraw>) -> Result<(), ProgramError> {
        ctx.accounts.withdraw_cnft(&ctx)
    }

    /// Withdraw two compressed NFTs from the vault PDA in a single transaction.
    #[instruction(discriminator = 1)]
    pub fn withdraw_two_cnfts(ctx: CtxWithRemaining<WithdrawTwo>) -> Result<(), ProgramError> {
        ctx.accounts.withdraw_two_cnfts(&ctx)
    }
}
