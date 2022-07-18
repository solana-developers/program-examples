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

    let mint_authority = &mut ctx.accounts.mint_authority;

    msg!("Creating token account...");
    msg!("Token Address: {}", &ctx.accounts.token_account.key());    
    associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.payer.to_account_info(),
                associated_token: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
    )?;

    msg!("Minting token to token account...");
    msg!("Mint: {}", &ctx.accounts.mint_account.to_account_info().key());   
    msg!("Token Address: {}", &ctx.accounts.token_account.key());     
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: mint_authority.to_account_info(),
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
    #[account(mut)]
    pub mint_account: Account<'info, token::Mint>,
    #[account(
        mut, 
        seeds = [b"mint_authority_", mint_account.key().as_ref()],
        bump = mint_authority_pda_bump
    )]
    pub mint_authority: Account<'info, MintAuthorityPda>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}