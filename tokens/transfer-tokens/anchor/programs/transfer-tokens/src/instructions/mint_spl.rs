use {
    anchor_lang::prelude::*,
    anchor_spl::{
        token,
        associated_token,
    },
};


pub fn mint_spl(
    ctx: Context<MintSpl>, 
    quantity: u64,
) -> Result<()> {

    msg!("Minting token to token account...");
    msg!("Mint: {}", &ctx.accounts.mint_account.to_account_info().key());   
    msg!("Token Address: {}", &ctx.accounts.associated_token_account.key());     
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        ),
        quantity,
    )?;

    msg!("Token minted to wallet successfully.");

    Ok(())
}


#[derive(Accounts)]
pub struct MintSpl<'info> {
    #[account(
        mut,
        mint::decimals = 9,
        mint::authority = mint_authority.key(),
    )]
    pub mint_account: Account<'info, token::Mint>,
    pub mint_authority: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub associated_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}