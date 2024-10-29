use transfer_sol_api::prelude::*;
use steel::*;


pub fn process_transfer_sol_with_cpi(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Parse accounts
    let [payer_info, recipient_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    payer_info.is_signer()?;
    system_program.is_program(&system_program::ID)?;

    // Parse instruction data
    let args = TransferArgs::try_from_bytes(data)?;

    // Execute transfer via CPI
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            payer_info.key,
            recipient_info.key,
            args.amount,
        ),
        &[
            payer_info.clone(),
            recipient_info.clone(),
            system_program.clone(),
        ],
    )
}

pub fn process_transfer_sol_with_program(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Parse accounts
    let [payer_info, recipient_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate account ownership
    payer_info.has_owner(&ID)?;

    // Parse instruction data
    let args = TransferArgs::try_from_bytes(data)?;

    // Transfer lamports directly
    let mut payer_lamports = payer_info.try_borrow_mut_lamports()?;
    let mut recipient_lamports = recipient_info.try_borrow_mut_lamports()?;

    **payer_lamports = payer_lamports
        .checked_sub(args.amount)
        .ok_or(TransferError::InvalidAmount)?;
    **recipient_lamports = recipient_lamports
        .checked_add(args.amount)
        .ok_or(TransferError::InvalidAmount)?;

    Ok(())
}