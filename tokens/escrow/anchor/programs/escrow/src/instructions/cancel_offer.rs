use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{close_account, transfer_checked, CloseAccount, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::Offer;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CancelOffer<'a> {
    #[account(mut)]
    pub maker: Signer<'a>,

    #[account(
        mut,
        close = maker,
        seeds = [b"offer", maker.key().as_ref(), &id.to_le_bytes()],
        bump = offer.bump,
        has_one = maker,
        has_one = token_mint_a
    )]
    pub offer: Account<'a, Offer>,

    #[account(
        mut,
        associated_token::authority = offer,
        associated_token::mint = token_mint_a,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'a, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_account_a: InterfaceAccount<'a, TokenAccount>,

    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'a, Mint>,

    pub token_program: Interface<'a, TokenInterface>,
    pub system_program: Program<'a, System>,
}

pub fn handler(ctx: Context<CancelOffer>, _id: u64) -> Result<()> {
    // 1. Transfer Token A from the vault back to the maker
    let seeds = &[
        b"offer",
        ctx.accounts.maker.to_account_info().key.as_ref(),
        &_id.to_le_bytes(),
        &[ctx.accounts.offer.bump],
    ];

    let signer_seeds = &[&seeds[..]];

    let transfer_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.maker_token_account_a.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
    };

    let cpi_cotext = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer_seeds,
    );

    transfer_checked(
        cpi_cotext,
        ctx.accounts.vault.amount,
        ctx.accounts.token_mint_a.decimals,
    )?;

    // 2. Close the vault account and reclaim rent to the maker
    let close_accounts = CloseAccount {
        account: ctx.accounts.vault.to_account_info(),
        destination: ctx.accounts.maker.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        close_accounts,
        signer_seeds,
    );
    close_account(cpi_context)?;
    Ok(())
}
