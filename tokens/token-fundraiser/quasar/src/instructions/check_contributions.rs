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

#[inline(always)]
pub fn handle_check_contributions(accounts: &mut CheckContributions, fundraiser_bump: u8) -> Result<(), ProgramError> {
    // Verify the target was met
    require!(
        accounts.fundraiser.current_amount >= accounts.fundraiser.amount_to_raise,
        ProgramError::Custom(0) // TargetNotMet
    );

    let maker_key = accounts.fundraiser.maker;
    let bump = [fundraiser_bump];
    let seeds: &[Seed] = &[
        Seed::from(b"fundraiser" as &[u8]),
        Seed::from(maker_key.as_ref()),
        Seed::from(&bump as &[u8]),
    ];

    // Transfer all vault funds to the maker
    let vault_amount = accounts.vault.amount();
    accounts.token_program
        .transfer(accounts.vault, accounts.maker_ta, accounts.fundraiser, vault_amount)
        .invoke_signed(seeds)?;

    // Close the vault token account
    accounts.token_program
        .close_account(accounts.vault, accounts.maker, accounts.fundraiser)
        .invoke_signed(seeds)?;

    Ok(())
}
