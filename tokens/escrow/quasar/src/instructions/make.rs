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

impl Make<'_> {
    #[inline(always)]
    pub fn make_escrow(&mut self, receive: u64, bumps: &MakeBumps) -> Result<(), ProgramError> {
        self.escrow.set_inner(
            *self.maker.address(),
            *self.mint_a.address(),
            *self.mint_b.address(),
            *self.maker_ta_b.address(),
            receive,
            bumps.escrow,
        );
        Ok(())
    }

    #[inline(always)]
    pub fn deposit_tokens(&mut self, amount: u64) -> Result<(), ProgramError> {
        self.token_program
            .transfer(self.maker_ta_a, self.vault_ta_a, self.maker, amount)
            .invoke()
    }
}
