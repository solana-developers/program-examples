use anchor_lang::prelude::*;

declare_id!("3EMcczaGi9ivdLxvvFwRbGYeEUEHpGwabXegARw4jLxa");

pub mod contexts;

pub use contexts::*;

#[program]
pub mod mint_nft {

    use super::*;
    pub fn create_collection(mut context: Context<CreateCollectionAccountConstraints>) -> Result<()> {
        handle_create_collection(&mut context.accounts, &context.bumps)
    }

    pub fn mint_nft(mut context: Context<MintNFTAccountConstraints>) -> Result<()> {
        handle_mint_nft(&mut context.accounts, &context.bumps)
    }

    pub fn verify_collection(mut context: Context<VerifyCollectionMintAccountConstraints>) -> Result<()> {
        handle_verify_collection(&mut context.accounts, &context.bumps)
    }
}
