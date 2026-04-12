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

impl Refund<'_> {
    #[inline(always)]
    pub fn withdraw_tokens_and_close(&mut self, bumps: &RefundBumps) -> Result<(), ProgramError> {
        let maker_key = self.escrow.maker;
        let bump = [bumps.escrow];
        let seeds: &[Seed] = &[
            Seed::from(b"escrow" as &[u8]),
            Seed::from(maker_key.as_ref()),
            Seed::from(&bump as &[u8]),
        ];

        self.token_program
            .transfer(
                self.vault_ta_a,
                self.maker_ta_a,
                self.escrow,
                self.vault_ta_a.amount(),
            )
            .invoke_signed(seeds)?;

        self.token_program
            .close_account(self.vault_ta_a, self.maker, self.escrow)
            .invoke_signed(seeds)?;
        Ok(())
    }
}
