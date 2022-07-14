use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke,
        system_program,
    },
    anchor_spl::{
        token,
        associated_token,
    },
    mpl_token_metadata::instruction as mpl_instruction,
    spl_token::instruction::AuthorityType,
};


declare_id!("AaWo2HWYXs5YtZoV4mPN1ZA8e8gb3wHqrXxQRxTWBJBC");


#[program]
pub mod mint_nft {
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
            (Rent::get()?).minimum_balance(token::Mint::LEN),
            token::Mint::LEN as u64,
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
            0,                                              // 0 Decimals (NFT)
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

        msg!("NFT mint created successfully.");

        msg!("Creating token account...");
        msg!("Token Address: {}", &ctx.accounts.token_account.key());    
        associated_token::create(
            CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                associated_token::Create {
                    payer: ctx.accounts.mint_authority.to_account_info(),
                    associated_token: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
        )?;

        msg!("Minting NFT to token account...");
        msg!("NFT Mint: {}", &ctx.accounts.mint_account.to_account_info().key());   
        msg!("Token Address: {}", &ctx.accounts.token_account.key());     
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint_account.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            1,
        )?;

        msg!("NFT minted to wallet successfully.");

        msg!("Disabling future minting...");
        msg!("NFT Mint: {}", &ctx.accounts.mint_account.to_account_info().key());   
        token::set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::SetAuthority {
                    current_authority: ctx.accounts.mint_authority.to_account_info(),
                    account_or_mint: ctx.accounts.mint_account.to_account_info(),
                },
            ),
            AuthorityType::MintTokens,
            None
        )?;

        msg!("NFT minting disabled successfully.");
        msg!("NFT mint process completed successfully.");

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
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}