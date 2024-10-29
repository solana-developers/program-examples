use pda_rent_payer_api::prelude::*;
use steel::*;

pub fn process_create_account(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // // Parse args.
    // let args = Add::try_from_bytes(data)?;
	// let amount = u64::from_le_bytes(args.amount);

    // Load and validate accounts.
    let [payer_info, new_account_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);        
    };
	new_account_info
        .is_signer()?
        .is_empty()?
        .is_writable()?;
    payer_info.is_writable()?.has_seeds(
        &[RENT_VAULT],
        &pda_rent_payer_api::ID,
    )?;
    system_program.is_program(&system_program::ID)?;

    // Create new account.
    create_account::<NewAccount>(
        new_account_info,
        system_program,
        payer_info,
        &pda_rent_payer_api::ID,
        &[RENT_VAULT],
    )?;

    Ok(())
}
