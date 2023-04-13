use {
    anchor_lang::{prelude::*, solana_program::program::invoke_signed},
    anchor_spl::token,
    mpl_token_metadata::instruction as mpl_instruction,
};

use crate::state::MintAuthorityPda;

pub fn create_token(
    ctx: Context<CreateToken>,
    nft_title: String,
    nft_symbol: String,
    nft_uri: String,
) -> Result<()> {
    let bump = ctx.accounts.mint_authority.bump;
    msg!("Creating metadata account...");
    msg!(
        "Metadata account address: {}",
        &ctx.accounts.metadata_account.key()
    );
    invoke_signed(
        &mpl_instruction::create_metadata_accounts_v3(
            ctx.accounts.token_metadata_program.key(), // Program ID (the Token Metadata Program)
            ctx.accounts.metadata_account.key(),       // Metadata account
            ctx.accounts.mint_account.key(),           // Mint account
            ctx.accounts.mint_authority.key(),         // Mint authority
            ctx.accounts.payer.key(),                  // Payer
            ctx.accounts.mint_authority.key(),         // Update authority
            nft_title,                                 // Name
            nft_symbol,                                // Symbol
            nft_uri,                                   // URI
            None,                                      // Creators
            0,                                         // Seller fee basis points
            true,                                      // Update authority is signer
            false,                                     // Is mutable
            None,                                      // Collection
            None,                                      // Uses
            None,                                      // Collection Details
        ),
        &[
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.mint_account.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        &[&[MintAuthorityPda::SEED_PREFIX.as_bytes(), &[bump]]],
    )?;

    msg!("NFT created successfully.");

    Ok(())
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = mint_authority.key(),
        mint::freeze_authority = mint_authority.key(),
    )]
    pub mint_account: Account<'info, token::Mint>,
    #[account(
        mut,
        seeds = [ MintAuthorityPda::SEED_PREFIX.as_bytes() ],
        bump = mint_authority.bump,
    )]
    pub mint_authority: Account<'info, MintAuthorityPda>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}
