use quasar_lang::prelude::*;

/// Accounts for creating a new system-owned account.
/// Both payer and new_account must sign the transaction.
#[derive(Accounts)]
pub struct CreateSystemAccount<'info> {
    #[account(mut)]
    pub payer: &'info Signer,
    #[account(mut)]
    pub new_account: &'info Signer,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_create_system_account(accounts: &CreateSystemAccount) -> Result<(), ProgramError> {
    // Create a zero-data account owned by the system program,
    // funded with the minimum rent-exempt balance.
    let system_program_address = Address::default();
    accounts.system_program
        .create_account_with_minimum_balance(
            accounts.payer,
            accounts.new_account,
            0, // space: zero bytes of data
            &system_program_address,
            None, // fetch Rent sysvar automatically
        )?
        .invoke()
}
