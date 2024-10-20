use account_data_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_create_address_info(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    msg!("Processing CreateAddressInfo instruction");

    // expected pda bump
    let bump = account_pda().1;

    // load accounts
    let [signer_account, create_address_account, system_program_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // validations
    signer_account.is_signer()?;
    create_address_account
        .is_empty()?
        .is_writable()?
        .has_seeds(&[ADDRESS_INFO_SEED], bump, &account_data_api::ID)?;
    system_program_account.is_program(&system_program::ID)?;

    // parse args.
    let args = CreateAddressInfo::try_from_bytes(data)?;
    msg!("Args: {:?}", args);

    // create account
    create_account::<AddressInfo>(
        create_address_account,
        &account_data_api::ID,
        &[ADDRESS_INFO_SEED, &[bump]],
        system_program_account,
        signer_account,
    )?;

    // initialize data
    let address_info =
        create_address_account.to_account_mut::<AddressInfo>(&account_data_api::ID)?;
    address_info.data = args.data;

    msg!("Address info updated: {:?}", address_info.data);

    Ok(())
}
