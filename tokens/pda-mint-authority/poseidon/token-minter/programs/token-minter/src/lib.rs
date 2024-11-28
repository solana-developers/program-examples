use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
declare_id!("2Ry3iUWABuQv8PTjgPwaM1CFHB8D8CtuX6EVzYXQ3PvE");
#[program]
pub mod token_minter {
    use super::*;
    pub fn create_token(_ctx: Context<CreateTokenContext>, _decimals: u8) -> Result<()> {
        // Note: Initialization for mint handled manually
        // As Poseidon's transpiler does not support initializeMint yet.

        Ok(())
    }
    pub fn mint_token(ctx: Context<MintTokenContext>, amount: u64) -> Result<()> {
        let signer_seeds: &[&[&[u8]]; 1] = &[&[b"mint", &[ctx.bumps.mint_account]]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.mint_account.to_account_info(),
            },
            signer_seeds,
        );
        mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateTokenContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    // Note: Poseidon's transpiler does not support initializeMint yet,
    // so this code is done manually using Anchor's InitializeMint.
    // init,
    // seeds = [b"mint"],
    // bump,
    // payer = payer,
    // mint::decimals = 9,
    // mint::authority = mint_account.key(),
    // mint::freeze_authority = mint_account.key(), this code is added manually
    #[account(
        init,
        seeds = [b"mint"],
        bump,
        payer = payer,
        mint::decimals = 9,
        mint::authority = mint_account.key(),
        mint::freeze_authority = mint_account.key(),

    )]
    pub mint_account: Account<'info, Mint>,
    // Token Program and System Program is added manually as Poseidon does not support it yet using initializeMint
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
#[derive(Accounts)]
pub struct MintTokenContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"mint"], bump)]
    pub mint_account: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
