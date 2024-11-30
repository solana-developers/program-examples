use account_data_steel_api::prelude::*;
use solana_program::rent::Rent;
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Parse data into AddressInfo.
    //
    let address_info_data = bytemuck::try_from_bytes::<AddressInfo>(data)
        .or(Err(ProgramError::InvalidInstructionData))?;

    let [payer, address_info_account, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    payer.is_signer()?;
    address_info_account.is_signer()?;
    address_info_account.is_empty()?;
    system_program.is_program(&system_program::ID)?;

    let rent = Rent::get()?;
    let space = 8 + std::mem::size_of::<AddressInfo>();

    // create the account
    //
    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            payer.key,
            address_info_account.key,
            rent.minimum_balance(space), // lamports
            space as u64,                // space
            program_id,                  // program id
        ),
        &[
            payer.clone(),
            address_info_account.clone(),
            system_program.clone(),
        ],
    )?;

    // parse the account to AddressInfo
    //
    let address_info = address_info_account.as_account_mut::<AddressInfo>(program_id)?;

    // set the account data
    //
    address_info.name = address_info_data.name;
    address_info.house_number = address_info_data.house_number;
    address_info.street = address_info_data.street;
    address_info.city = address_info_data.city;

    Ok(())
}
