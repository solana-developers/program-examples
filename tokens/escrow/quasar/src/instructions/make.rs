use {
    crate::state::Escrow,
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token, TokenCpi},
};

#[derive(Accounts)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: &'info Signer,
    #[account(mut, init, payer = maker, seeds = [b"escrow", maker], bump)]
    pub escrow: &'info mut Account<Escrow>,
    pub mint_a: &'info Account<Mint>,
    pub mint_b: &'info Account<Mint>,
    #[account(mut)]
    pub maker_ta_a: &'info mut Account<Token>,
    #[account(mut, init_if_needed, payer = maker, token::mint = mint_b, token::authority = maker)]
    pub maker_ta_b: &'info mut Account<Token>,
    #[account(mut, init_if_needed, payer = maker, token::mint = mint_a, token::authority = escrow)]
    pub vault_ta_a: &'info mut Account<Token>,
    pub rent: &'info Sysvar<Rent>,
    pub token_program: &'info Program<Token>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_make_escrow(accounts: &mut Make, receive: u64, bumps: &MakeBumps) -> Result<(), ProgramError> {
    accounts.escrow.set_inner(
        *accounts.maker.address(),
        *accounts.mint_a.address(),
        *accounts.mint_b.address(),
        *accounts.maker_ta_b.address(),
        receive,
        bumps.escrow,
    );
    Ok(())
}

#[inline(always)]
pub fn handle_deposit_tokens(accounts: &mut Make, amount: u64) -> Result<(), ProgramError> {
    accounts.token_program
        .transfer(accounts.maker_ta_a, accounts.vault_ta_a, accounts.maker, amount)
        .invoke()
}
