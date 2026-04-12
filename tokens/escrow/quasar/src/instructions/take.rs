use {
    crate::state::Escrow,
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token, TokenCpi},
};

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: &'info Signer,
    #[account(
        mut,
        has_one = maker,
        has_one = maker_ta_b,
        constraint = escrow.receive > 0,
        close = taker,
        seeds = [b"escrow", maker],
        bump = escrow.bump
    )]
    pub escrow: &'info mut Account<Escrow>,
    #[account(mut)]
    pub maker: &'info UncheckedAccount,
    pub mint_a: &'info Account<Mint>,
    pub mint_b: &'info Account<Mint>,
    #[account(mut, init_if_needed, payer = taker, token::mint = mint_a, token::authority = taker)]
    pub taker_ta_a: &'info mut Account<Token>,
    #[account(mut)]
    pub taker_ta_b: &'info mut Account<Token>,
    #[account(mut, init_if_needed, payer = taker, token::mint = mint_b, token::authority = maker)]
    pub maker_ta_b: &'info mut Account<Token>,
    #[account(mut)]
    pub vault_ta_a: &'info mut Account<Token>,
    pub rent: &'info Sysvar<Rent>,
    pub token_program: &'info Program<Token>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_transfer_tokens(accounts: &mut Take) -> Result<(), ProgramError> {
    accounts.token_program
        .transfer(
            accounts.taker_ta_b,
            accounts.maker_ta_b,
            accounts.taker,
            accounts.escrow.receive,
        )
        .invoke()
}

#[inline(always)]
pub fn handle_withdraw_tokens_and_close(accounts: &mut Take, bumps: &TakeBumps) -> Result<(), ProgramError> {
    let maker_key = accounts.escrow.maker;
    let bump = [bumps.escrow];
    let seeds: &[Seed] = &[
        Seed::from(b"escrow" as &[u8]),
        Seed::from(maker_key.as_ref()),
        Seed::from(&bump as &[u8]),
    ];

    accounts.token_program
        .transfer(
            accounts.vault_ta_a,
            accounts.taker_ta_a,
            accounts.escrow,
            accounts.vault_ta_a.amount(),
        )
        .invoke_signed(seeds)?;

    accounts.token_program
        .close_account(accounts.vault_ta_a, accounts.taker, accounts.escrow)
        .invoke_signed(seeds)?;
    Ok(())
}
