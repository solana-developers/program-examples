use anchor_lang::prelude::*;

pub use anchor_lang::solana_program::sysvar::instructions::ID as INSTRUCTIONS_ID;
use anchor_spl::metadata::mpl_token_metadata::instructions::{
    VerifyCpi, VerifyCpiAccounts, VerifyInstructionArgs,
};
use anchor_spl::metadata::mpl_token_metadata::types::VerificationArgs;
use anchor_spl::metadata::{MasterEditionAccount, MetadataAccount};
use anchor_spl::{metadata::Metadata, token::Mint};

#[derive(Accounts)]
pub struct VerifyCollectionMint<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub metadata: Account<'info, MetadataAccount>,
    pub mint: Account<'info, Mint>,
    #[account(
        seeds = [b"authority"],
        bump,
    )]
    /// CHECK: This account is not initialized and is being used for signing purposes only
    pub mint_authority: UncheckedAccount<'info>,
    pub collection_mint: Account<'info, Mint>,
    #[account(mut)]
    pub collection_metadata: Account<'info, MetadataAccount>,
    pub collection_master_edition: Account<'info, MasterEditionAccount>,
    pub system_program: Program<'info, System>,
    #[account(address = INSTRUCTIONS_ID)]
    /// CHECK: Sysvar instruction account that is being checked with an address constraint
    pub sysvar_instruction: UncheckedAccount<'info>,
    pub token_metadata_program: Program<'info, Metadata>,
}

impl<'info> VerifyCollectionMint<'info> {
    pub fn verify_collection(&mut self, bumps: &VerifyCollectionMintBumps) -> Result<()> {
        let metadata = &self.metadata.to_account_info();
        let authority = &self.mint_authority.to_account_info();
        let collection_mint = &self.collection_mint.to_account_info();
        let collection_metadata = &self.collection_metadata.to_account_info();
        let collection_master_edition = &self.collection_master_edition.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let sysvar_instructions = &self.sysvar_instruction.to_account_info();
        let spl_metadata_program = &self.token_metadata_program.to_account_info();

        let seeds = &[&b"authority"[..], &[bumps.mint_authority]];
        let signer_seeds = &[&seeds[..]];

        // Invoke CPI
        let verify_cpi = VerifyCpi::new(
            spl_metadata_program,
            VerifyCpiAccounts {
                authority,
                delegate_record: None,
                metadata,
                collection_mint: Some(collection_mint),
                collection_metadata: Some(collection_metadata),
                collection_master_edition: Some(collection_master_edition),
                system_program,
                sysvar_instructions,
            },
            VerifyInstructionArgs {
                verification_args: VerificationArgs::CollectionV1,
            },
        );

        verify_cpi.invoke_signed(signer_seeds)?;

        msg!("Collection Verified!");

        Ok(())
    }
}
