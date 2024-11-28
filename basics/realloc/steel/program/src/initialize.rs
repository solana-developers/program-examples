use realloc_api::prelude::*;
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let args = Initialize::try_from_bytes(data)?;
    let len = u32::from_le_bytes(args.len);
    
    let [payer_info, message_account_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    payer_info.is_signer()?;
    message_account_info.is_signer()?;
    
    // Create the account
    let space = Message::required_space(len as usize);
    
    create_account::<Message>(
        message_account_info,
        system_program,
        payer_info,
        &realloc_api::ID,
        &[],
    )?;

    // Initialize the message
    let message = message_account_info.as_account_mut::<Message>(&realloc_api::ID)?;
    message.message = args.message;
    message.len = len;

    Ok(())
}
