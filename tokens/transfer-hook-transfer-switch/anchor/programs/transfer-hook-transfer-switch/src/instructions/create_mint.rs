use crate::{constants::*, error::*, state::*};
use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{mint_to, Mint, MintTo, Token, TokenAccount},
    },
};

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(
        init,
        payer = authority,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = authority,
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [STATE_SEED],
        bump = state.bump,
        has_one = authority @ TransferHookError::InvalidAuthority,
    )]
    pub state: Account<'info, TransferHookState>,

    // Add ATA for initial mint
    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub authority_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
    msg!("Creating new mint...");
    msg!("Mint address: {}", ctx.accounts.mint.key());
    msg!("Authority: {}", ctx.accounts.authority.key());
    msg!(
        "Authority token account: {}",
        ctx.accounts.authority_token.key()
    );

    // Mint initial supply to authority
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.authority_token.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        INITIAL_MINT_AMOUNT * 10u64.pow(TOKEN_DECIMALS as u32),
    )?;

    msg!("Initial supply minted to authority");
    Ok(())
}
