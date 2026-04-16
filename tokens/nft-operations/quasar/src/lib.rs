#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;
#[cfg(test)]
mod tests;

declare_id!("22222222222222222222222222222222222222222222");

/// NFT operations: create a collection, mint NFTs into it, and verify
/// collection membership.
///
/// Uses a PDA (`["authority"]`) as the mint authority and update authority
/// for both the collection and individual NFTs.
#[program]
mod quasar_nft_operations {
    use super::*;

    /// Create a collection NFT: mint, metadata, and master edition.
    #[instruction(discriminator = 0)]
    pub fn create_collection(ctx: Ctx<CreateCollection>) -> Result<(), ProgramError> {
        instructions::handle_create_collection(&mut ctx.accounts, &ctx.bumps)
    }

    /// Mint an individual NFT with a reference to the collection.
    #[instruction(discriminator = 1)]
    pub fn mint_nft(ctx: Ctx<MintNft>) -> Result<(), ProgramError> {
        instructions::handle_mint_nft(&mut ctx.accounts, &ctx.bumps)
    }

    /// Verify the NFT as a member of the collection.
    #[instruction(discriminator = 2)]
    pub fn verify_collection(ctx: Ctx<VerifyCollectionMint>) -> Result<(), ProgramError> {
        instructions::handle_verify_collection(&mut ctx.accounts, &ctx.bumps)
    }
}
