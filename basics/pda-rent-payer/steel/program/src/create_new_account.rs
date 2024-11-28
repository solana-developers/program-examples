use pda_rent_payer_api::prelude::*;
use solana_program::msg;
use steel::sysvar::rent::Rent;
use steel::*;

pub fn process_create_account(accounts: &[AccountInfo<'_>]) -> ProgramResult {
    // Load accounts
    let [rent_vault_info, new_account_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    new_account_info.is_signer()?.is_empty()?.is_writable()?;
    rent_vault_info
        .is_writable()?
        .has_seeds(&[RENT_VAULT], &pda_rent_payer_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    let vault_balance = rent_vault_info.lamports();

    msg!("Vault balance: {}", vault_balance);
    // First we get the lamports required for rent
    // assuming this account has no inner data
    let lamports_required_for_rent = (Rent::get()?).minimum_balance(0);

    if vault_balance < lamports_required_for_rent {
        return Err(ProgramError::InsufficientFunds);
    }

    // Then we create a new account by simply sending a
    // token amount to the new account.
    rent_vault_info.send(lamports_required_for_rent, new_account_info);

    msg!("Created new account.");
    msg!("New account: {:?}", new_account_info.key);

    Ok(())
}
