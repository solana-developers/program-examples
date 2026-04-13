use anchor_lang::prelude::*;
use anchor_spl::token::{
    transfer, 
    Mint, 
    Token, 
    TokenAccount, 
    Transfer
};

use crate::{
    state::{
        Contributor, 
        Fundraiser
    }, 
    SECONDS_TO_DAYS
};

#[derive(Accounts)]
pub struct RefundAccountConstraints<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,
    pub maker: SystemAccount<'info>,
    pub mint_to_raise: Account<'info, Mint>,
    #[account(
        mut,
        has_one = mint_to_raise,
        seeds = [b"fundraiser", maker.key().as_ref()],
        bump = fundraiser.bump,
    )]
    pub fundraiser: Account<'info, Fundraiser>,
    #[account(
        mut,
        seeds = [b"contributor", fundraiser.key().as_ref(), contributor.key().as_ref()],
        bump,
        close = contributor,
    )]
    pub contributor_account: Account<'info, Contributor>,
    #[account(
        mut,
        associated_token::mint = mint_to_raise,
        associated_token::authority = contributor
    )]
    pub contributor_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_to_raise,
        associated_token::authority = fundraiser
    )]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle_refund(accounts: &mut RefundAccountConstraints) -> Result<()> {

        // Check if the fundraising duration has been reached
        let current_time = Clock::get()?.unix_timestamp;
 
        require!(
            accounts.fundraiser.duration >= ((current_time - accounts.fundraiser.time_started) / SECONDS_TO_DAYS) as u16,
            crate::FundraiserError::FundraiserNotEnded
        );

        require!(
            accounts.vault.amount < accounts.fundraiser.amount_to_raise,
            crate::FundraiserError::TargetMet
        );

        // Transfer the funds back to the contributor
        // CPI to the token program to transfer the funds
        let cpi_program = accounts.token_program.key();

        // Transfer the funds from the vault to the contributor
        let cpi_accounts = Transfer {
            from: accounts.vault.to_account_info(),
            to: accounts.contributor_ata.to_account_info(),
            authority: accounts.fundraiser.to_account_info(),
        };

        // Signer seeds to sign the CPI on behalf of the fundraiser account
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"fundraiser".as_ref(),
            accounts.maker.to_account_info().key.as_ref(),
            &[accounts.fundraiser.bump],
        ]];

        // CPI context with signer since the fundraiser account is a PDA
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        // Transfer the funds from the vault to the contributor
        transfer(cpi_ctx, accounts.contributor_account.amount)?;

        // Update the fundraiser state by reducing the amount contributed
        accounts.fundraiser.current_amount -= accounts.contributor_account.amount;

        Ok(())
    }
