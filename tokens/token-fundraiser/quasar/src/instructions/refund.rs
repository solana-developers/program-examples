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

impl Refund<'_> {
    #[inline(always)]
    pub fn refund(&mut self, fundraiser_bump: u8) -> Result<(), ProgramError> {
        let refund_amount = self.contributor_account.amount;

        let maker_key = self.fundraiser.maker;
        let bump = [fundraiser_bump];
        let seeds: &[Seed] = &[
            Seed::from(b"fundraiser" as &[u8]),
            Seed::from(maker_key.as_ref()),
            Seed::from(&bump as &[u8]),
        ];

        // Transfer contributor's tokens back from vault
        self.token_program
            .transfer(self.vault, self.contributor_ta, self.fundraiser, refund_amount)
            .invoke_signed(seeds)?;

        // Update fundraiser state
        self.fundraiser.current_amount = self.fundraiser.current_amount
            .checked_sub(refund_amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Zero out contributor amount
        self.contributor_account.set_inner(0);

        Ok(())
    }
}
