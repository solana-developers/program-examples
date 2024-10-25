use account_data_api::prelude::*;
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, account_data_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Deserialize instruction aguments.
    let args = Initialize::try_from_bytes(data)?;

    // Derive PDA bump seed.
    let (_, bump) = address_info_pda(*signer_info.key);

    // Check signer account is instruction signer.
    signer_info.is_signer()?;

    // Check account data account is empty, writable, and matches seeds.
    account_data_info.is_empty()?.is_writable()?.has_seeds(
        &[ACCOUNT_DATA_SEED, signer_info.key.as_ref()],
        &account_data_api::ID,
    )?;

    // Check system program account is the system program.
    system_program.is_program(&system_program::ID)?;

    // Call CPI to create account data account.
    create_account::<AddressInfoData>(
        account_data_info,
        system_program,
        signer_info,
        &account_data_api::ID,
        &[ACCOUNT_DATA_SEED, signer_info.key.as_ref()],
    )?;

    // Initialize account data account.
    let account_data =
        account_data_info.as_account_mut::<AddressInfoData>(&account_data_api::ID)?;

    // Set account data fields.
    account_data.name = args.name;
    account_data.house_number = args.house_number;
    account_data.street = args.street;
    account_data.city = args.city;
    account_data.bump = bump;

    Ok(())
}
