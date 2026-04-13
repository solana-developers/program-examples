use anchor_lang::prelude::*;
use anchor_lang::solana_program::rent::{
    DEFAULT_EXEMPTION_THRESHOLD, DEFAULT_LAMPORTS_PER_BYTE_YEAR,
};
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token_interface::{
    token_metadata_initialize, Mint, Token2022, TokenMetadataInitialize,
};
use spl_token_metadata_interface::state::TokenMetadata;
use spl_type_length_value::variable_len_pack::VariableLenPack;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 2,
        mint::authority = payer,
        extensions::metadata_pointer::authority = payer,
        extensions::metadata_pointer::metadata_address = mint_account,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn handle_process_initialize(context: Context<Initialize>, args: TokenMetadataArgs) -> Result<()> {
    let TokenMetadataArgs { name, symbol, uri } = args;

    // Define token metadata
    let token_metadata = TokenMetadata {
        name: name.clone(),
        symbol: symbol.clone(),
        uri: uri.clone(),
        ..Default::default()
    };

    // Add 4 extra bytes for size of MetadataExtension (2 bytes for type, 2 bytes for length)
    let data_len = 4 + token_metadata.get_packed_len()?;

    // Calculate lamports required for the additional metadata
    let lamports =
        data_len as u64 * DEFAULT_LAMPORTS_PER_BYTE_YEAR * DEFAULT_EXEMPTION_THRESHOLD as u64;

    // Transfer additional lamports to mint account
    transfer(
        CpiContext::new(
            context.accounts.system_program.key(),
            Transfer {
                from: context.accounts.payer.to_account_info(),
                to: context.accounts.mint_account.to_account_info(),
            },
        ),
        lamports,
    )?;

    // Initialize token metadata
    token_metadata_initialize(
        CpiContext::new(
            context.accounts.token_program.key(),
            TokenMetadataInitialize {
                token_program_id: context.accounts.token_program.to_account_info(),
                mint: context.accounts.mint_account.to_account_info(),
                metadata: context.accounts.mint_account.to_account_info(),
                mint_authority: context.accounts.payer.to_account_info(),
                update_authority: context.accounts.payer.to_account_info(),
            },
        ),
        name,
        symbol,
        uri,
    )?;
    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct TokenMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
