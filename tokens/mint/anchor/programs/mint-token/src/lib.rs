use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke,
        system_program,
    },
    anchor_spl::token,
    mpl_token_metadata::{
        ID as TOKEN_METADATA_ID,
        instruction as mpl_instruction,
    },
};


declare_id!("3hmFGLkjJc7TfSLdEYYYwGAz6KsrD8anYwtSkF6TYjBU");


#[program]
pub mod mint_token {
    use super::*;

    pub fn mint_token(
        ctx: Context<MintToken>, 
        metadata_title: String, 
        metadata_symbol: String, 
        metadata_uri: String,
    ) -> Result<()> {

        msg!("Creating mint account...");
        msg!("Mint: {}", &ctx.accounts.mint_account.key());
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.mint_authority.to_account_info(),
                    to: ctx.accounts.mint_account.to_account_info(),
                },
            ),
            10000000,
            82,
            &ctx.accounts.token_program.key(),
        )?;

        msg!("Initializing mint account...");
        msg!("Mint: {}", &ctx.accounts.mint_account.key());
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint_account.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            9,
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key()),
        )?;

        msg!("Creating metadata account...");
        msg!("Metadata account address: {}", &ctx.accounts.metadata_account.key());
        invoke(
            &mpl_instruction::create_metadata_accounts_v2(
                TOKEN_METADATA_ID, 
                ctx.accounts.metadata_account.key(), 
                ctx.accounts.mint_account.key(), 
                ctx.accounts.mint_authority.key(), 
                ctx.accounts.mint_authority.key(), 
                ctx.accounts.mint_authority.key(), 
                metadata_title, 
                metadata_symbol, 
                metadata_uri, 
                None,
                1,
                true, 
                false, 
                None, 
                None,
            ),
            &[
                ctx.accounts.metadata_account.to_account_info(),
                ctx.accounts.mint_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!("Token mint process completed successfully.");

        Ok(())
    }
}


#[derive(Accounts)]
pub struct MintToken<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_account: Signer<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}