use steel_api::prelude::*;
use steel::*;

pub fn process_initialize_vault(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load and validate accounts.
    let [signer_info, rent_vault_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);        
    };
    signer_info.is_signer()?;
    rent_vault_info.is_empty()?.is_writable()?.has_seeds(
        &[RENT_VAULT],
        &pda_rent_payer_api::ID
    )?;
    system_program.is_program(&system_program::ID)?;

    // Initialize counter.
    create_account::<Counter>(
        rent_vault_info,
        system_program,
        signer_info,
        &pda_rent_payer_api::ID,
        &[RENT_VAULT],
    )?;
    let rent_vault = rent_vault_info.as_account_mut::<RentVault>(&pda_rent_payer_api::ID)?;

    Ok(())
}
