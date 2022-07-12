use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke,
        system_program,
    },
    anchor_spl::token,
    mpl_token_metadata::instruction as mpl_instruction,
};


declare_id!("4Bg2L3bHNk2wPszETtqE76hJHVXmnw2pqeUuumSSx7in");


#[program]
pub mod mint_nft {
    use super::*;

    pub fn mint_token(
        ctx: Context<MintNft>, 
        metadata_title: String, 
        metadata_symbol: String, 
        metadata_uri: String,
    ) -> Result<()> {

        const MINT_SIZE: u64 = 82;

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
            (Rent::get()?).minimum_balance(MINT_SIZE as usize),
            MINT_SIZE,
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
            0,                                              // 0 Decimals
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key()),
        )?;

        msg!("Creating metadata account...");
        msg!("Metadata account address: {}", &ctx.accounts.metadata_account.key());
        invoke(
            &mpl_instruction::create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),      // Program ID (the Token Metadata Program)
                ctx.accounts.metadata_account.key(),            // Metadata account
                ctx.accounts.mint_account.key(),                // Mint account
                ctx.accounts.mint_authority.key(),              // Mint authority
                ctx.accounts.mint_authority.key(),              // Payer
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
                ctx.accounts.token_metadata_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!("Token mint process completed successfully.");

        Ok(())
    }
}


#[derive(Accounts)]
pub struct MintNft<'info> {
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