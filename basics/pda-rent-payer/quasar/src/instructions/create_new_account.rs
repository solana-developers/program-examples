use quasar_lang::prelude::*;

/// Accounts for creating a new account funded by the rent vault PDA.
/// The rent vault signs the create_account CPI via PDA seeds.
#[derive(Accounts)]
pub struct CreateNewAccount<'info> {
    #[account(mut)]
    pub new_account: &'info Signer,
    #[account(mut, seeds = [b"rent_vault"], bump)]
    pub rent_vault: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_create_new_account(accounts: &CreateNewAccount, rent_vault_bump: u8) -> Result<(), ProgramError> {
    // Build PDA signer seeds: ["rent_vault", bump].
    let bump_bytes = [rent_vault_bump];
    let seeds: &[Seed] = &[
        Seed::from(b"rent_vault" as &[u8]),
        Seed::from(&bump_bytes as &[u8]),
    ];

    let system_program_address = Address::default();

    // Create a zero-data system-owned account, funded from the vault.
    accounts.system_program
        .create_account_with_minimum_balance(
            accounts.rent_vault,
            accounts.new_account,
            0, // space: zero bytes of data
            &system_program_address,
            None, // fetch Rent sysvar automatically
        )?
        .invoke_signed(seeds)
}
