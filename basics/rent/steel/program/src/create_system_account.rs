use rent_api::prelude::*;
use steel::*;
use solana_program::sysvar::rent::Rent;
use solana_program::*;

pub fn process_create_system_account(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Parse accounts
    let [payer_info, new_account_info, system_program] = accounts else {
        msg!("❌ Missing required accounts");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    payer_info.is_signer()?;
    new_account_info.is_signer()?;
    system_program.is_program(&system_program::ID)?;

    // Parse instruction data
    let args = CreateSystemAccountArgs::try_from_bytes(data)?;
    
    msg!("Program invoked. Creating a system account...");
    msg!("New public key will be: {}", new_account_info.key);
    msg!("Name length: {}", args.name_len);
    msg!("Address length: {}", args.address_len);

    // Calculate account size and required lamports
    let account_size = std::mem::size_of::<AddressData>();
    let lamports_required = Rent::get()?.minimum_balance(account_size);

    msg!("Account size: {}", account_size);
    msg!("Lamports required: {}", lamports_required);

    // Create the account
    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            payer_info.key,
            new_account_info.key,
            lamports_required,
            account_size as u64,
            &rent_api::ID,
        ),
        &[
            payer_info.clone(),
            new_account_info.clone(),
            system_program.clone(),
        ],
    )?;

    msg!("Account created successfully, initializing data...");

    // Initialize the account data
    let mut account_data = new_account_info.try_borrow_mut_data()?;
    let address_data = AddressData {
        name_len: args.name_len,
        name: args.name,
        address_len: args.address_len,
        address: args.address,
    };
    
    msg!("Writing data to account...");
    let serialized = address_data.to_bytes();
    account_data.copy_from_slice(&serialized);

    msg!("✅ Account initialized with provided data");
    msg!("Name length stored: {}", address_data.name_len);
    msg!("Address length stored: {}", address_data.address_len);

    Ok(())
}
