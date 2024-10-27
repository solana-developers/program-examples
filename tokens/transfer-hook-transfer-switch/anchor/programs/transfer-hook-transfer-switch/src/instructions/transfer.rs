use crate::{constants::*, error::*, events::*, state::*};
use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub source_authority: Signer<'info>,

    /// CHECK: Destination authority checked in token account constraint
    pub destination_authority: SystemAccount<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        seeds = [WALLET_STATE_SEED, source_authority.key().as_ref(), mint.key().as_ref()],
        bump = wallet_state.bump,
        has_one = mint,
        constraint = wallet_state.owner == source_authority.key() @ TransferHookError::InvalidAuthority
    )]
    pub wallet_state: Account<'info, WalletState>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = source_authority,
    )]
    pub source_token: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = source_authority,
        associated_token::mint = mint,
        associated_token::authority = destination_authority,
    )]
    pub destination_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
    msg!("Processing transfer...");
    msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());
    msg!("Source Token Account: {}", &ctx.accounts.source_token.key());
    msg!(
        "Destination Token Account: {}",
        &ctx.accounts.destination_token.key()
    );

    // Check if wallet is frozen
    require!(
        !ctx.accounts.wallet_state.is_frozen,
        TransferHookError::WalletFrozen
    );

    // Check sufficient funds
    require!(
        ctx.accounts.source_token.amount >= amount,
        TransferHookError::InsufficientFunds
    );

    // Transfer tokens with decimal adjustment
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.source_token.to_account_info(),
                to: ctx.accounts.destination_token.to_account_info(),
                authority: ctx.accounts.source_authority.to_account_info(),
            },
        ),
        amount * 10u64.pow(ctx.accounts.mint.decimals as u32),
    )?;

    // Emit transfer event
    emit!(TransferProcessed {
        source: ctx.accounts.source_token.key(),
        destination: ctx.accounts.destination_token.key(),
        amount,
        mint: ctx.accounts.mint.key()
    });

    msg!("Transfer completed successfully");
    Ok(())
}
