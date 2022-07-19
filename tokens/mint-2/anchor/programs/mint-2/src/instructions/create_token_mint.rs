use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke_signed,
    },
    anchor_spl::token,
    mpl_token_metadata::instruction as mpl_instruction,
};


pub fn create_token_mint(
    ctx: Context<CreateTokenMint>, 
    metadata_title: String, 
    metadata_symbol: String, 
    metadata_uri: String,
    mint_authority_pda_bump: u8,
) -> Result<()> {

    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", &ctx.accounts.metadata_account.key());
    invoke_signed(
        &mpl_instruction::create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),      // Program ID (the Token Metadata Program)
            ctx.accounts.metadata_account.key(),            // Metadata account
            ctx.accounts.mint_account.key(),                // Mint account
            ctx.accounts.mint_authority.key(),              // Mint authority
            ctx.accounts.payer.key(),                       // Payer
            ctx.accounts.mint_authority.key(),              // Update authority
            metadata_title,                                 // Name
            metadata_symbol,                                // Symbol
            metadata_uri,                                   // URI
            None,                                           // Creators
            0,                                              // Seller fee basis points
            true,                                           // Update authority is signer
            false,                                          // Is mutable
            None,                                           // Collection
            None,                                           // Uses
        ),
        &[
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.mint_account.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        &[
            &[
                b"mint_authority_", 
                ctx.accounts.mint_account.key().as_ref(),
                &[mint_authority_pda_bump],
            ]
        ]
    )?;

    msg!("Token mint created successfully.");

    Ok(())
}


#[derive(Accounts)]
pub struct CreateTokenMint<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = mint_authority.key(),
    )]
    pub mint_account: Account<'info, token::Mint>,
    #[account(
        init, 
        payer = payer,
        space = 8 + 32,
        seeds = [
            b"mint_authority_", 
            mint_account.key().as_ref(),
        ],
        bump
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

#[account]
pub struct MintAuthorityPda {}