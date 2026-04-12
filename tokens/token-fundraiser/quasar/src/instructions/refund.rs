use {
    crate::state::{Contributor, Fundraiser},
    quasar_lang::prelude::*,
    quasar_spl::{Token, TokenCpi},
};

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub contributor: &'info Signer,
    pub maker: &'info UncheckedAccount,
    #[account(
        mut,
        has_one = maker,
        seeds = [b"fundraiser", maker],
        bump = fundraiser.bump
    )]
    pub fundraiser: &'info mut Account<Fundraiser>,
    #[account(mut)]
    pub contributor_account: &'info mut Account<Contributor>,
    #[account(mut)]
    pub contributor_ta: &'info mut Account<Token>,
    #[account(mut)]
    pub vault: &'info mut Account<Token>,
    pub token_program: &'info Program<Token>,
}

#[inline(always)]
pub fn handle_refund(accounts: &mut Refund, fundraiser_bump: u8) -> Result<(), ProgramError> {
    let refund_amount = accounts.contributor_account.amount;

    let maker_key = accounts.fundraiser.maker;
    let bump = [fundraiser_bump];
    let seeds: &[Seed] = &[
        Seed::from(b"fundraiser" as &[u8]),
        Seed::from(maker_key.as_ref()),
        Seed::from(&bump as &[u8]),
    ];

    // Transfer contributor's tokens back from vault
    accounts.token_program
        .transfer(accounts.vault, accounts.contributor_ta, accounts.fundraiser, refund_amount)
        .invoke_signed(seeds)?;

    // Update fundraiser state
    accounts.fundraiser.current_amount = accounts.fundraiser.current_amount
        .checked_sub(refund_amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Zero out contributor amount
    accounts.contributor_account.set_inner(0);

    Ok(())
}
