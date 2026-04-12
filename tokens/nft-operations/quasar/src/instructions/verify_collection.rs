use quasar_lang::prelude::*;
use quasar_spl::metadata::{MetadataCpi, MetadataProgram};

/// Accounts for verifying an NFT as part of a collection.
///
/// Uses `verify_sized_collection_item` which is the Metaplex Token Metadata
/// instruction for verifying collection membership on sized collections.
///
/// The Anchor version uses typed `MetadataAccount` / `MasterEditionAccount`
/// wrappers for owner and discriminant validation. In Quasar we use
/// `UncheckedAccount` and rely on the Metaplex program itself to validate
/// the accounts during CPI — the on-chain program enforces correctness.
#[derive(Accounts)]
pub struct VerifyCollectionMint<'info> {
    pub authority: &'info Signer,
    /// The NFT's metadata account (will be updated with verified=true).
    #[account(mut)]
    pub metadata: &'info UncheckedAccount,
    /// PDA used as collection authority.
    #[account(seeds = [b"authority"], bump)]
    pub mint_authority: &'info UncheckedAccount,
    /// The collection mint.
    pub collection_mint: &'info UncheckedAccount,
    /// The collection's metadata account.
    #[account(mut)]
    pub collection_metadata: &'info UncheckedAccount,
    /// The collection's master edition account.
    pub collection_master_edition: &'info UncheckedAccount,
    pub system_program: &'info Program<System>,
    pub token_metadata_program: &'info MetadataProgram,
}

impl VerifyCollectionMint<'_> {
    #[inline(always)]
    pub fn verify_collection(
        &self,
        bumps: &VerifyCollectionMintBumps,
    ) -> Result<(), ProgramError> {
        let bump = [bumps.mint_authority];
        let seeds: &[Seed] = &[
            Seed::from(b"authority" as &[u8]),
            Seed::from(&bump as &[u8]),
        ];

        self.token_metadata_program
            .verify_sized_collection_item(
                self.metadata,
                self.mint_authority,
                self.authority, // payer
                self.collection_mint,
                self.collection_metadata,
                self.collection_master_edition,
            )
            .invoke_signed(seeds)?;

        log("Collection Verified!");
        Ok(())
    }
}
