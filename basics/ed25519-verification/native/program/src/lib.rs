use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    ed25519_program,
};

entrypoint!(process_instruction);

#[derive(Debug)]
pub struct TransferInstruction {
    pub amount: u64,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    
    // Get account info
    let custodial_account = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;
    let signer = next_account_info(accounts_iter)?;
    let ed25519_program_id = next_account_info(accounts_iter)?;

    // Verify this is the expected Ed25519 program
    if ed25519_program_id.key != &ed25519_program::id() {
        return Err(ProgramError::InvalidArgument);
    }

    // First 64 bytes are the signature
    let signature = &instruction_data[..64];
    // Next 32 bytes are the public key
    let public_key = &instruction_data[64..96];
    // Next 8 bytes are the amount
    let amount = u64::from_le_bytes(instruction_data[96..104].try_into().unwrap());
    // Remaining data is the message to verify
    let message = &instruction_data[104..];

    // Verify the Ed25519 signature
    let verification_instruction = ed25519_program::instruction::new_ed25519_instruction(
        public_key,
        message,
        signature,
    );

    // Invoke the Ed25519 program to verify the signature
    solana_program::program::invoke(
        &verification_instruction,
        &[ed25519_program_id.clone()],
    )?;

    msg!("Signature verification successful!");

    // Transfer funds from custodial account to recipient
    **custodial_account.try_borrow_mut_lamports()? = custodial_account
        .lamports()
        .checked_sub(amount)
        .ok_or(ProgramError::InsufficientFunds)?;

    **recipient.try_borrow_mut_lamports()? = recipient
        .lamports()
        .checked_add(amount)
        .ok_or(ProgramError::Overflow)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_transfer_with_valid_signature() {
        // Test implementation here
    }
} 