use realloc_api::prelude::*;
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Parse accounts
    let [payer_info, message_account_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    payer_info.is_signer()?;
    system_program.is_program(&system_program::ID)?;

    // Parse instruction data
    let args = Initialize::try_from_bytes(data)?;

    // Calculate required space
    let required_space = Message::required_space(args.message_len as usize);

    // Create account with exact required space
    create_account::<Message>(
        message_account_info,
        system_program,
        payer_info,
        &ID,
        &[b"message"],
    )?;

    // Get mutable reference to account data and update
    let message = message_account_info.as_account_mut::<Message>(&ID)?;
    message.message_len = args.message_len;
    message.message[..args.message_len as usize].copy_from_slice(&args.message[..args.message_len as usize]);

    Ok(())
}
