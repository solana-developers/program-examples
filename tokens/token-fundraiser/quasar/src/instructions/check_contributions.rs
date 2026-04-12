use {
    crate::state::Fundraiser,
    quasar_lang::prelude::*,
    quasar_spl::{Token, TokenCpi},
};

#[derive(Accounts)]
pub struct CheckContributions<'info> {
    #[account(mut)]
    pub maker: &'info Signer,
    #[account(
        mut,
        has_one = maker,
        close = maker,
        seeds = [b"fundraiser", maker],
        bump = fundraiser.bump
    )]
    pub fundraiser: &'info mut Account<Fundraiser>,
    #[account(mut)]
    pub vault: &'info mut Account<Token>,
    #[account(mut)]
    pub maker_ta: &'info mut Account<Token>,
    pub token_program: &'info Program<Token>,
}

impl CheckContributions<'_> {
    #[inline(always)]
    pub fn check_contributions(&mut self, fundraiser_bump: u8) -> Result<(), ProgramError> {
        // Verify the target was met
        require!(
            self.fundraiser.current_amount >= self.fundraiser.amount_to_raise,
            ProgramError::Custom(0) // TargetNotMet
        );

        let maker_key = self.fundraiser.maker;
        let bump = [fundraiser_bump];
        let seeds: &[Seed] = &[
            Seed::from(b"fundraiser" as &[u8]),
            Seed::from(maker_key.as_ref()),
            Seed::from(&bump as &[u8]),
        ];

        // Transfer all vault funds to the maker
        let vault_amount = self.vault.amount();
        self.token_program
            .transfer(self.vault, self.maker_ta, self.fundraiser, vault_amount)
            .invoke_signed(seeds)?;

        // Close the vault token account
        self.token_program
            .close_account(self.vault, self.maker, self.fundraiser)
            .invoke_signed(seeds)?;

        Ok(())
    }
}
