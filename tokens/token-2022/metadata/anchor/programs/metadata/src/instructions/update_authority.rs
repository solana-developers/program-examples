use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    spl_pod::optional_keys::OptionalNonZeroPubkey, token_metadata_update_authority, Mint,
    Token2022, TokenMetadataUpdateAuthority,
};

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    pub current_authority: Signer<'info>,
    pub new_authority: Option<UncheckedAccount<'info>>,

    #[account(
        mut,
        extensions::metadata_pointer::metadata_address = mint_account,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn handle_process_update_authority(context: Context<UpdateAuthority>) -> Result<()> {
    let new_authority_key = match &context.accounts.new_authority {
        Some(account) => OptionalNonZeroPubkey::try_from(Some(account.key()))?,
        None => OptionalNonZeroPubkey::try_from(None)?,
    };

    // Change update authority
    token_metadata_update_authority(
        CpiContext::new(
            context.accounts.token_program.key(),
            TokenMetadataUpdateAuthority {
                token_program_id: context.accounts.token_program.to_account_info(),
                metadata: context.accounts.mint_account.to_account_info(),
                current_authority: context.accounts.current_authority.to_account_info(),

                // new authority isn't actually needed as account in the CPI
                // using current_authority as a placeholder to satisfy the struct
                new_authority: context.accounts.current_authority.to_account_info(),
            },
        ),
        new_authority_key,
    )?;
    Ok(())
}
