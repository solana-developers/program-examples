use account_data_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_create_address_info(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    msg!("Processing CreateAddressInfo instruction");

    let [signer_account, create_address_account, system_program_account] = accounts else {
        msg!("Error: Missing required accounts");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let (_expected_address, bump) = account_pda();

    // Validate accounts
    if !signer_account.is_signer {
        msg!("Error: Signer account must sign the transaction");
        return Err(ProgramError::MissingRequiredSignature);
    }

    create_address_account
        .is_empty()
        .map_err(|_| {
            msg!("Error: Address account must be empty");
            ProgramError::AccountAlreadyInitialized
        })?
        .is_writable()
        .map_err(|_| {
            msg!("Error: Address account must be writable");
            ProgramError::InvalidAccountData
        })?
        .has_seeds(&[ADDRESS_INFO_SEED], bump, &account_data_api::ID)
        .map_err(|_| {
            msg!("Error: Invalid PDA seeds or bump");
            ProgramError::InvalidSeeds
        })?;

    system_program_account.is_program(&system_program::ID)?;

    let args = parse_instruction_data(data)?;
    msg!(
        "Instruction data parsed successfully: {{ 
            name: '{}', 
            house_number: {}, 
            street: '{}', 
            city: '{}' 
        }}",
        bytes_to_string(&args.data.name),
        u64::from_le_bytes(args.data.house_number),
        bytes_to_string(&args.data.street),
        bytes_to_string(&args.data.city),
    );

    create_account::<AddressInfo>(
        create_address_account,
        &account_data_api::ID,
        &[ADDRESS_INFO_SEED, &[bump]],
        system_program_account,
        signer_account,
    )?;

    let address_info = create_address_account
        .to_account_mut::<AddressInfo>(&account_data_api::ID)
        .map_err(|_| {
            msg!("Error: Failed to deserialize address info account");
            ProgramError::InvalidAccountData
        })?;

    address_info.data = args.data;
    msg!(
        "Address info updated successfully: {{ 
            name: '{}', 
            house_number: {}, 
            street: '{}', 
            city: '{}' 
        }}",
        bytes_to_string(&address_info.data.name),
        u64::from_le_bytes(address_info.data.house_number),
        bytes_to_string(&address_info.data.street),
        bytes_to_string(&address_info.data.city),
    );

    msg!("CreateAddressInfo instruction completed successfully");
    Ok(())
}

fn parse_instruction_data(data: &[u8]) -> Result<CreateAddressInfo, ProgramError> {
    CreateAddressInfo::try_from_bytes(data)
        .map(|info| info.to_owned())
        .map_err(|_| {
            msg!("Error: Failed to parse instruction data");
            ProgramError::InvalidInstructionData
        })
}

// Helper function to convert byte array to string
fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8(
        bytes
            .iter()
            .take_while(|&&b| b != 0)
            .copied()
            .collect::<Vec<u8>>(),
    )
    .unwrap_or_default()
}
