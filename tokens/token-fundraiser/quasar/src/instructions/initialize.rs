use {
    crate::state::Fundraiser,
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub maker: &'info Signer,
    pub mint_to_raise: &'info Account<Mint>,
    #[account(mut, init, payer = maker, seeds = [b"fundraiser", maker], bump)]
    pub fundraiser: &'info mut Account<Fundraiser>,
    #[account(mut, init_if_needed, payer = maker, token::mint = mint_to_raise, token::authority = fundraiser)]
    pub vault: &'info mut Account<Token>,
    pub rent: &'info Sysvar<Rent>,
    pub token_program: &'info Program<Token>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_initialize(
    accounts: &mut Initialize, amount_to_raise: u64,
    duration: u16,
    bump: u8,
) -> Result<(), ProgramError> {
    // Validate minimum raise amount
    require!(amount_to_raise > 0, ProgramError::InvalidArgument);

    accounts.fundraiser.set_inner(
        *accounts.maker.address(),
        *accounts.mint_to_raise.address(),
        amount_to_raise,
        0,  // current_amount starts at 0
        0,  // time_started — would be Clock::get() on-chain
        duration,
        bump,
    );
    Ok(())
}
