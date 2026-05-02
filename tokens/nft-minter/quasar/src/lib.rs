#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;
use quasar_spl::{
    metadata::{MetadataCpi, MetadataProgram},
    Mint, Token, TokenCpi,
};

#[cfg(test)]
mod tests;

declare_id!("22222222222222222222222222222222222222222222");

/// NFT minter: creates a mint (decimals = 0), mints 1 token, creates Metaplex
/// metadata and master edition in a single instruction.
#[program]
mod quasar_nft_minter {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn mint_nft(
        ctx: Ctx<MintNft>,
        nft_name: String,
        nft_symbol: String,
        nft_uri: String,
    ) -> Result<(), ProgramError> {
        handle_mint_nft(&mut ctx.accounts, &nft_name, &nft_symbol, &nft_uri)
    }
}

/// All accounts needed to mint an NFT in one transaction.
#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub payer: &'info Signer,

    /// Metadata PDA — initialised by the Metaplex program.
    #[account(mut)]
    pub metadata_account: &'info UncheckedAccount,

    /// Master edition PDA — initialised by the Metaplex program.
    #[account(mut)]
    pub edition_account: &'info UncheckedAccount,

    /// NFT mint (decimals = 0).
    #[account(mut, init, payer = payer, mint::decimals = 0, mint::authority = payer, mint::freeze_authority = payer)]
    pub mint_account: &'info mut Account<Mint>,

    /// Token account holding the NFT.
    #[account(mut, init_if_needed, payer = payer, token::mint = mint_account, token::authority = payer)]
    pub associated_token_account: &'info mut Account<Token>,

    pub token_program: &'info Program<Token>,
    pub token_metadata_program: &'info MetadataProgram,
    pub system_program: &'info Program<System>,
    pub rent: &'info Sysvar<Rent>,
}

#[inline(always)]
pub fn handle_mint_nft(
    accounts: &MintNft, nft_name: &str,
    nft_symbol: &str,
    nft_uri: &str,
) -> Result<(), ProgramError> {
    // 1. Mint one token to the associated token account.
    log("Minting token");
    accounts.token_program
        .mint_to(
            accounts.mint_account,
            accounts.associated_token_account,
            accounts.payer,
            1u64,
        )
        .invoke()?;

    // 2. Create Metaplex metadata account.
    log("Creating metadata account");
    accounts.token_metadata_program
        .create_metadata_accounts_v3(
            accounts.metadata_account,
            accounts.mint_account,
            accounts.payer,
            accounts.payer,
            accounts.payer,
            accounts.system_program,
            accounts.rent,
            nft_name,
            nft_symbol,
            nft_uri,
            0,     // seller_fee_basis_points
            false, // is_mutable
            true,  // update_authority_is_signer
        )
        .invoke()?;

    // 3. Create master edition (makes it a verified NFT).
    log("Creating master edition account");
    accounts.token_metadata_program
        .create_master_edition_v3(
            accounts.edition_account,
            accounts.mint_account,
            accounts.payer, // update_authority
            accounts.payer, // mint_authority
            accounts.payer, // payer
            accounts.metadata_account,
            accounts.token_program,
            accounts.system_program,
            accounts.rent,
            None, // max_supply = unlimited
        )
        .invoke()?;

    log("NFT minted successfully.");
    Ok(())
}
