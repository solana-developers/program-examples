use realloc_api::prelude::*;
use steel::*;

pub fn process_update(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let args = Update::try_from_bytes(data)?;
    let len = u32::from_le_bytes(args.len);
    
    let [payer_info, message_account_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    payer_info.is_signer()?;
    
    // Update the message
    let message = message_account_info.as_account_mut::<Message>(&realloc_api::ID)?;
    message.message = args.message;
    message.len = len;

    Ok(())
}