use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke,
    },
    anchor_spl::token,
    mpl_token_metadata::instruction as mpl_instruction,
};


declare_id!("5yRmjtx87UJMJF4NEeqjpmgAu7MBJZACW6ksiCYqQxVh");


#[program]
pub mod create_token {
    use super::*;

    pub fn create_token_mint(
        ctx: Context<CreateTokenMint>, 
        token_title: String, 
        token_symbol: String, 
        token_uri: String,
        _token_decimals: u8,
    ) -> Result<()> {
    
        msg!("Creating metadata account...");
        msg!("Metadata account address: {}", &ctx.accounts.metadata_account.key());
        invoke(
            &mpl_instruction::create_metadata_accounts_v3(
                ctx.accounts.token_metadata_program.key(),      // Program ID (the Token Metadata Program)
                ctx.accounts.metadata_account.key(),            // Metadata account
                ctx.accounts.mint_account.key(),                // Mint account
                ctx.accounts.mint_authority.key(),              // Mint authority
                ctx.accounts.payer.key(),                       // Payer
                ctx.accounts.mint_authority.key(),              // Update authority
                token_title,                                    // Name
                token_symbol,                                   // Symbol
                token_uri,                                      // URI
                None,                                           // Creators
                0,                                              // Seller fee basis points
                true,                                           // Update authority is signer
                false,                                          // Is mutable
                None,                                           // Collection
                None,                                           // Uses
                None,                                           // Collection Details
            ),
            &[
                ctx.accounts.metadata_account.to_account_info(),
                ctx.accounts.mint_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ]
        )?;
    
        msg!("Token mint created successfully.");
    
        Ok(())
    }
}


// The macros within the Account Context will create our
//      Mint account and initialize it as a Mint
//      We just have to do the metadata
//
#[derive(Accounts)]
#[instruction(
    token_title: String, 
    token_symbol: String, 
    token_uri: String,
    token_decimals: u8,
)]
pub struct CreateTokenMint<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = token_decimals,
        mint::authority = mint_authority.key(),
    )]
    pub mint_account: Account<'info, token::Mint>,
    pub mint_authority: SystemAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}