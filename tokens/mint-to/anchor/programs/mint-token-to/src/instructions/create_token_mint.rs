use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke_signed,
        system_program,
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

    let mint_authority = &mut ctx.accounts.mint_authority;

    msg!("Creating mint account...");
    msg!("Mint: {}", &ctx.accounts.mint_account.key());
    system_program::create_account(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            system_program::CreateAccount {
                from: ctx.accounts.payer.to_account_info(),
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
        9,                                              // 9 Decimals
        &mint_authority.key(),
        Some(&mint_authority.key()),
    )?;

    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", &ctx.accounts.metadata_account.key());
    invoke_signed(
        &mpl_instruction::create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),      // Program ID (the Token Metadata Program)
            ctx.accounts.metadata_account.key(),            // Metadata account
            ctx.accounts.mint_account.key(),                // Mint account
            mint_authority.key(),              // Mint authority
            ctx.accounts.payer.key(),              // Payer
            mint_authority.key(),              // Update authority
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
            mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            mint_authority.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        &[&[
            b"mint_authority_", 
            ctx.accounts.mint_account.key().as_ref(),
            &[mint_authority_pda_bump],
        ]]
    )?;

    msg!("Token mint created successfully.");

    Ok(())
}


#[derive(Accounts)]
pub struct CreateTokenMint<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_account: Signer<'info>,
    #[account(
        init, 
        payer = payer,
        space = 8 + 32,
        seeds = [b"mint_authority_", mint_account.key().as_ref()],
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