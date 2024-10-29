use pda_rent_payer_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_initialize_vault(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args
    let args = data[..8].try_into().expect("Error parsing args");
    let amount = u64::from_le_bytes(args);

    // Load and validate accounts.
    let [payer_info, rent_vault_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    payer_info.is_signer()?;
    rent_vault_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[RENT_VAULT], &pda_rent_payer_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Initialize vault.
    create_account::<RentVault>(
        rent_vault_info,
        system_program,
        payer_info,
        &pda_rent_payer_api::ID,
        &[RENT_VAULT],
    )?;

    payer_info.send(amount, rent_vault_info);

    rent_vault_info.collect(amount, payer_info)?;

    let _ = rent_vault_info.as_account_mut::<RentVault>(&pda_rent_payer_api::ID)?;

    msg!("Initialized rent vault.");
    msg!("PDA: {:?}", rent_vault_info.key);

    Ok(())
}
