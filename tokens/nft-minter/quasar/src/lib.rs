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
        ctx.accounts.mint_nft(&nft_name, &nft_symbol, &nft_uri)
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

impl MintNft<'_> {
    #[inline(always)]
    pub fn mint_nft(
        &self,
        nft_name: &str,
        nft_symbol: &str,
        nft_uri: &str,
    ) -> Result<(), ProgramError> {
        // 1. Mint one token to the associated token account.
        log("Minting token");
        self.token_program
            .mint_to(
                self.mint_account,
                self.associated_token_account,
                self.payer,
                1u64,
            )
            .invoke()?;

        // 2. Create Metaplex metadata account.
        log("Creating metadata account");
        self.token_metadata_program
            .create_metadata_accounts_v3(
                self.metadata_account,
                self.mint_account,
                self.payer,
                self.payer,
                self.payer,
                self.system_program,
                self.rent,
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
        self.token_metadata_program
            .create_master_edition_v3(
                self.edition_account,
                self.mint_account,
                self.payer, // update_authority
                self.payer, // mint_authority
                self.payer, // payer
                self.metadata_account,
                self.token_program,
                self.system_program,
                self.rent,
                None, // max_supply = unlimited
            )
            .invoke()?;

        log("NFT minted successfully.");
        Ok(())
    }
}
