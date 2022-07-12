use {
    std::convert::TryInto,
    solana_program::{
        account_info::{
            next_account_info, AccountInfo
        },
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        program::invoke,
        program_error::ProgramError,
        pubkey::Pubkey,
        system_instruction,
    },
};


entrypoint!(process_instruction);


fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;

    let amount = instruction_data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    msg!("Received request to transfer {:?} lamports from {:?} to {:?}.", 
        amount, payer.key, recipient.key);
    msg!("  Processing transfer...");

    invoke(
        &system_instruction::transfer(payer.key, recipient.key, amount),
        &[payer.clone(), recipient.clone()],
    )?;
    
    msg!("Transfer completed successfully.");
    Ok(())
}