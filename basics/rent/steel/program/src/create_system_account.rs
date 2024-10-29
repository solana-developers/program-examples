use rent_api::prelude::*;
use steel::*;
use solana_program::*;
pub fn process_create_system_account(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Parse accounts
    let [payer_info, new_account_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    payer_info.is_signer()?;
    new_account_info.is_signer()?;
    system_program.is_program(&system_program::ID)?;

    // Parse instruction data
    let args = CreateSystemAccountArgs::try_from_bytes(data)?;

    // Log information
    msg!("Program invoked. Creating a system account...");
    msg!("New public key will be: {}", new_account_info.key);

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

    msg!("Account created successfully.");

    Ok(())
}
