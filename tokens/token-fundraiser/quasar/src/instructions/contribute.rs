use {
    crate::state::{Contributor, Fundraiser},
    quasar_lang::prelude::*,
    quasar_spl::{Token, TokenCpi},
};

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub contributor: &'info Signer,
    #[account(mut)]
    pub fundraiser: &'info mut Account<Fundraiser>,
    #[account(mut)]
    pub contributor_account: &'info mut Account<Contributor>,
    #[account(mut)]
    pub contributor_ta: &'info mut Account<Token>,
    #[account(mut)]
    pub vault: &'info mut Account<Token>,
    pub token_program: &'info Program<Token>,
}

impl Contribute<'_> {
    #[inline(always)]
    pub fn contribute(&mut self, amount: u64) -> Result<(), ProgramError> {
        require!(amount > 0, ProgramError::InvalidArgument);

        // Transfer tokens from contributor to vault
        self.token_program
            .transfer(self.contributor_ta, self.vault, self.contributor, amount)
            .invoke()?;

        // Update fundraiser state
        self.fundraiser.current_amount = self.fundraiser.current_amount.checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Update contributor tracking
        self.contributor_account.amount = self.contributor_account.amount.checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(())
    }
}
