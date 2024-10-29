use realloc_api::prelude::*;
use steel::*;
use solana_program::rent::Rent;

pub fn process_update(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Parse accounts
    let [payer_info, message_account_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    payer_info.is_signer()?;
    message_account_info.has_owner(&ID)?;
    system_program.is_program(&system_program::ID)?;

    // Parse instruction data
    let args = Update::try_from_bytes(data)?;

    // Calculate new required space
    let new_space = Message::required_space(args.message_len as usize);
    let old_space = message_account_info.data_len();

    // Only realloc if space needs to change
    if new_space != old_space {
        // Calculate additional rent needed (if expanding)
        let rent = Rent::get()?;
        let old_minimum_balance = rent.minimum_balance(old_space);
        let new_minimum_balance = rent.minimum_balance(new_space);

        // If expanding, transfer additional rent from payer
        if new_space > old_space {
            let additional_rent = new_minimum_balance.saturating_sub(old_minimum_balance);

            if additional_rent > 0 {
                solana_program::program::invoke(
                    &solana_program::system_instruction::transfer(
                        payer_info.key,
                        message_account_info.key,
                        additional_rent,
                    ),
                    &[
                        payer_info.clone(),
                        message_account_info.clone(),
                        system_program.clone(),
                    ],
                )?;
            }
        }

        // Perform realloc
        message_account_info.realloc(new_space, true)?;

        // If shrinking, return excess rent to payer
        if new_space < old_space {
            let excess_rent = old_minimum_balance.saturating_sub(new_minimum_balance);
            if excess_rent > 0 {
                **message_account_info.try_borrow_mut_lamports()? -= excess_rent;
                **payer_info.try_borrow_mut_lamports()? += excess_rent;
            }
        }
    }

    // Update message data
    let message = message_account_info.as_account_mut::<Message>(&ID)?;
    message.message_len = args.message_len;
    message.message[..args.message_len as usize].copy_from_slice(&args.message[..args.message_len as usize]);

    Ok(())
}
