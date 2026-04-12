use anchor_lang::prelude::*;

use anchor_spl::metadata::mpl_token_metadata::instructions::{
    VerifyCollectionV1Cpi,
    VerifyCollectionV1CpiAccounts,
};
use anchor_spl::metadata::{
    MasterEditionAccount, 
    MetadataAccount,
};
use anchor_spl::{
    token::Mint, 
    metadata::Metadata, 
};
// In Anchor 1.0, sysvar::instructions::ID moved — use the well-known address directly
const INSTRUCTIONS_SYSVAR_ID: Pubkey = anchor_lang::solana_program::pubkey::pubkey!("Sysvar1nstructions1111111111111111111111111");

#[derive(Accounts)]
pub struct VerifyCollectionMintAccountConstraints<'info> {
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
    #[account(address = INSTRUCTIONS_SYSVAR_ID)]
    /// CHECK: Sysvar instruction account that is being checked with an address constraint
    pub sysvar_instruction: UncheckedAccount<'info>,
    pub token_metadata_program: Program<'info, Metadata>,
}

pub fn handle_verify_collection(accounts: &mut VerifyCollectionMintAccountConstraints, bumps: &VerifyCollectionMintAccountConstraintsBumps) -> Result<()> {
        let metadata = &accounts.metadata.to_account_info();
        let authority = &accounts.mint_authority.to_account_info();
        let collection_mint = &accounts.collection_mint.to_account_info();
        let collection_metadata = &accounts.collection_metadata.to_account_info();
        let collection_master_edition = &accounts.collection_master_edition.to_account_info();
        let system_program = &accounts.system_program.to_account_info();
        let sysvar_instructions = &accounts.sysvar_instruction.to_account_info();
        let spl_metadata_program = &accounts.token_metadata_program.to_account_info();

        let seeds = &[
            &b"authority"[..], 
            &[bumps.mint_authority]
        ];
        let signer_seeds = &[&seeds[..]];

        let verify_collection = VerifyCollectionV1Cpi::new(
            spl_metadata_program,
        VerifyCollectionV1CpiAccounts {
            authority,
            delegate_record: None,
            metadata,
            collection_mint,
            collection_metadata: Some(collection_metadata),
            collection_master_edition: Some(collection_master_edition),
            system_program,
            sysvar_instructions,
        });
        verify_collection.invoke_signed(signer_seeds)?;

        msg!("Collection Verified!");
        
        Ok(())
    }
