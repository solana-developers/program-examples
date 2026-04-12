use quasar_lang::prelude::*;
use quasar_spl::{
    metadata::{MetadataCpi, MetadataProgram},
    Mint, Token, TokenCpi,
};

/// Accounts for minting an individual NFT with a collection reference.
#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub owner: &'info Signer,
    #[account(mut, init, payer = owner, mint::decimals = 0, mint::authority = mint_authority, mint::freeze_authority = mint_authority)]
    pub mint: &'info mut Account<Mint>,
    /// Token account to hold the NFT.
    #[account(mut, init_if_needed, payer = owner, token::mint = mint, token::authority = owner)]
    pub destination: &'info mut Account<Token>,
    /// Metadata PDA — initialised by the Metaplex program.
    #[account(mut)]
    pub metadata: &'info UncheckedAccount,
    /// Master edition PDA — initialised by the Metaplex program.
    #[account(mut)]
    pub master_edition: &'info UncheckedAccount,
    /// PDA used as mint authority and update authority.
    #[account(seeds = [b"authority"], bump)]
    pub mint_authority: &'info UncheckedAccount,
    /// The collection mint (must already exist).
    #[account(mut)]
    pub collection_mint: &'info Account<Mint>,
    pub system_program: &'info Program<System>,
    pub token_program: &'info Program<Token>,
    pub token_metadata_program: &'info MetadataProgram,
    pub rent: &'info Sysvar<Rent>,
}

impl MintNft<'_> {
    #[inline(always)]
    pub fn mint_nft(&self, bumps: &MintNftBumps) -> Result<(), ProgramError> {
        let bump = [bumps.mint_authority];
        let seeds: &[Seed] = &[
            Seed::from(b"authority" as &[u8]),
            Seed::from(&bump as &[u8]),
        ];

        // Mint 1 token to the destination.
        self.token_program
            .mint_to(self.mint, self.destination, self.mint_authority, 1u64)
            .invoke_signed(seeds)?;
        log("NFT minted!");

        // Create metadata with collection reference.
        // Note: The collection is set as unverified here; call verify_collection
        // separately to verify it.
        self.token_metadata_program
            .create_metadata_accounts_v3(
                self.metadata,
                self.mint,
                self.mint_authority,
                self.owner,
                self.mint_authority,
                self.system_program,
                self.rent,
                "Mint Test",
                "YAY",
                "",
                0,    // seller_fee_basis_points
                true, // is_mutable
                true, // update_authority_is_signer
            )
            .invoke_signed(seeds)?;

        // Create master edition.
        self.token_metadata_program
            .create_master_edition_v3(
                self.master_edition,
                self.mint,
                self.mint_authority, // update_authority
                self.mint_authority, // mint_authority
                self.owner,          // payer
                self.metadata,
                self.token_program,
                self.system_program,
                self.rent,
                Some(0), // max_supply = 0 means unique 1/1
            )
            .invoke_signed(seeds)?;

        Ok(())
    }
}
