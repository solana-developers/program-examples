use quasar_lang::prelude::*;
use quasar_spl::{
    metadata::{MetadataCpi, MetadataProgram},
    Mint, Token, TokenCpi,
};

/// Accounts for creating a collection NFT.
///
/// The PDA `["authority"]` acts as mint authority and update authority.
#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub user: &'info Signer,
    #[account(mut, init, payer = user, mint::decimals = 0, mint::authority = mint_authority, mint::freeze_authority = mint_authority)]
    pub mint: &'info mut Account<Mint>,
    /// PDA used as mint authority and update authority.
    #[account(seeds = [b"authority"], bump)]
    pub mint_authority: &'info UncheckedAccount,
    /// Metadata PDA — initialised by the Metaplex program.
    #[account(mut)]
    pub metadata: &'info UncheckedAccount,
    /// Master edition PDA — initialised by the Metaplex program.
    #[account(mut)]
    pub master_edition: &'info UncheckedAccount,
    /// Token account to hold the collection NFT.
    #[account(mut, init_if_needed, payer = user, token::mint = mint, token::authority = user)]
    pub destination: &'info mut Account<Token>,
    pub system_program: &'info Program<System>,
    pub token_program: &'info Program<Token>,
    pub token_metadata_program: &'info MetadataProgram,
    pub rent: &'info Sysvar<Rent>,
}

impl CreateCollection<'_> {
    #[inline(always)]
    pub fn create_collection(
        &self,
        bumps: &CreateCollectionBumps,
    ) -> Result<(), ProgramError> {
        let bump = [bumps.mint_authority];
        let seeds: &[Seed] = &[
            Seed::from(b"authority" as &[u8]),
            Seed::from(&bump as &[u8]),
        ];

        // Mint 1 token to the destination.
        self.token_program
            .mint_to(self.mint, self.destination, self.mint_authority, 1u64)
            .invoke_signed(seeds)?;
        log("Collection NFT minted!");

        // Create metadata account.
        self.token_metadata_program
            .create_metadata_accounts_v3(
                self.metadata,
                self.mint,
                self.mint_authority,
                self.user,
                self.mint_authority,
                self.system_program,
                self.rent,
                "DummyCollection",
                "DC",
                "",
                0,    // seller_fee_basis_points
                true, // is_mutable
                true, // update_authority_is_signer
            )
            .invoke_signed(seeds)?;
        log("Metadata Account created!");

        // Create master edition.
        self.token_metadata_program
            .create_master_edition_v3(
                self.master_edition,
                self.mint,
                self.mint_authority, // update_authority
                self.mint_authority, // mint_authority
                self.user,           // payer
                self.metadata,
                self.token_program,
                self.system_program,
                self.rent,
                Some(0), // max_supply = 0 means unique 1/1
            )
            .invoke_signed(seeds)?;
        log("Master Edition Account created");

        Ok(())
    }
}
