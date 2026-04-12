use quasar_lang::prelude::*;
use quasar_spl::{
    metadata::{MetadataCpi, MetadataProgram},
    Mint, Token,
};

/// Accounts for creating a new token mint with Metaplex metadata.
///
/// The mint is initialised via Quasar's `#[account(init)]`. The metadata
/// PDA is created by CPI-ing into the Metaplex Token Metadata program.
#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    #[account(
        mut,
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint_account: &'info mut Account<Mint>,
    /// The metadata PDA — will be initialised by the Metaplex program.
    #[account(mut)]
    pub metadata_account: &'info UncheckedAccount,
    pub token_program: &'info Program<Token>,
    pub token_metadata_program: &'info MetadataProgram,
    pub system_program: &'info Program<System>,
    pub rent: &'info Sysvar<Rent>,
}

#[inline(always)]
pub fn handle_create_token(
    accounts: &CreateToken, token_name: &str,
    token_symbol: &str,
    token_uri: &str,
) -> Result<(), ProgramError> {
    log("Creating metadata account");

    accounts.token_metadata_program
        .create_metadata_accounts_v3(
            accounts.metadata_account,
            accounts.mint_account,
            accounts.payer, // mint_authority
            accounts.payer, // payer
            accounts.payer, // update_authority
            accounts.system_program,
            accounts.rent,
            token_name,
            token_symbol,
            token_uri,
            0,     // seller_fee_basis_points
            false, // is_mutable
            true,  // update_authority_is_signer
        )
        .invoke()?;

    log("Token created successfully.");
    Ok(())
}
