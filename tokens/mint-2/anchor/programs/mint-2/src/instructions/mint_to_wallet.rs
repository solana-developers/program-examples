use {
    anchor_lang::prelude::*,
    anchor_spl::{
        token,
        associated_token,
    },
};
use crate::create_token_mint::MintAuthorityPda;


pub fn mint_to_wallet(
    ctx: Context<MintToWallet>, 
    amount: u64,
    mint_authority_pda_bump: u8,
) -> Result<()> {

    msg!("Minting token to token account...");
    msg!("Mint: {}", &ctx.accounts.mint_account.to_account_info().key());   
    msg!("Token Address: {}", &ctx.accounts.token_account.key());     
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_authority_", 
                ctx.accounts.mint_account.key().as_ref(),
                &[mint_authority_pda_bump],
            ]]
        ),
        amount,
    )?;

    msg!("Token minted to wallet successfully.");

    Ok(())
}


#[derive(Accounts)]
#[instruction(amount: u64, mint_authority_pda_bump: u8)]
pub struct MintToWallet<'info> {
    #[account(
        mut,
        mint::decimals = 9,
        mint::authority = mint_authority.key(),
    )]
    pub mint_account: Account<'info, token::Mint>,
    #[account(
        mut, 
        seeds = [
            b"mint_authority_", 
            mint_account.key().as_ref()
        ],
        bump = mint_authority_pda_bump
    )]
    pub mint_authority: Account<'info, MintAuthorityPda>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}