use {
    crate::state::Escrow,
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token, TokenCpi},
};

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: &'info Signer,
    #[account(
        mut,
        has_one = maker,
        close = maker,
        seeds = [b"escrow", maker],
        bump = escrow.bump
    )]
    pub escrow: &'info mut Account<Escrow>,
    pub mint_a: &'info Account<Mint>,
    #[account(mut, init_if_needed, payer = maker, token::mint = mint_a, token::authority = maker)]
    pub maker_ta_a: &'info mut Account<Token>,
    #[account(mut)]
    pub vault_ta_a: &'info mut Account<Token>,
    pub rent: &'info Sysvar<Rent>,
    pub token_program: &'info Program<Token>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_withdraw_tokens_and_close(accounts: &mut Refund, bumps: &RefundBumps) -> Result<(), ProgramError> {
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
            accounts.maker_ta_a,
            accounts.escrow,
            accounts.vault_ta_a.amount(),
        )
        .invoke_signed(seeds)?;

    accounts.token_program
        .close_account(accounts.vault_ta_a, accounts.maker, accounts.escrow)
        .invoke_signed(seeds)?;
    Ok(())
}
