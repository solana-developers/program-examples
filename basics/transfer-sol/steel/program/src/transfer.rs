use transfer_sol_api::prelude::*;
use steel::*;
use transfer_sol_api::ID;
use solana_program::msg;

pub fn process_transfer_sol_with_cpi(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    msg!("Entering process_transfer_sol_with_cpi");
    msg!("Instruction data length: {}", data.len());
    msg!("Raw instruction data: {:?}", data);
    
    // Log account info
    msg!("Number of accounts provided: {}", accounts.len());
    for (i, account) in accounts.iter().enumerate() {
        msg!("Account {}: {}", i, account.key);
        msg!("Is signer: {}", account.is_signer);
        msg!("Is writable: {}", account.is_writable);
        msg!("Owner: {}", account.owner);
    }

    // Parse accounts with detailed error
    let [payer_info, recipient_info, system_program] = accounts else {
        msg!("❌ Account parsing failed. Expected 3 accounts, got {}", accounts.len());
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts with logging
    if !payer_info.is_signer {
        msg!("❌ Payer must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    system_program.is_program(&system_program::ID)?;  // This will propagate the error if program check fails
    msg!("✅ System program verified: {}", system_program.key);

    // Parse instruction data with detailed error handling
    msg!("Attempting to parse TransferSolWithCpi data");
    let args = match TransferSolWithCpi::try_from_bytes(data) {
        Ok(parsed) => {
            msg!("✅ Successfully parsed transfer amount: {}", parsed.amount);
            parsed
        },
        Err(err) => {
            msg!("❌ Failed to parse args data");
            msg!("Error: {:?}", err);
            msg!("Expected format: TransferSolWithCpi with u64 amount");
            msg!("Got args bytes: {:?}", data);
            return Err(ProgramError::InvalidInstructionData);
        }
    };

    msg!("Executing CPI transfer of {} lamports", args.amount);
    
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
    msg!("Entering process_transfer_sol_with_program");
    msg!("Instruction data length: {}", data.len());
    msg!("Raw instruction data: {:?}", data);
    
    // Log account info
    msg!("Number of accounts provided: {}", accounts.len());
    for (i, account) in accounts.iter().enumerate() {
        msg!("Account {}: {}", i, account.key);
        msg!("Is signer: {}", account.is_signer);
        msg!("Is writable: {}", account.is_writable);
        msg!("Owner: {}", account.owner);
    }

    // Parse accounts with detailed error
    let [payer_info, recipient_info] = accounts else {
        msg!("❌ Account parsing failed. Expected 2 accounts, got {}", accounts.len());
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate account ownership with logging
    payer_info.has_owner(&ID)?;  // This will propagate the error if ownership check fails
    msg!("✅ Payer ownership verified: {}", payer_info.owner);

    // Parse instruction data with detailed error handling
    msg!("Attempting to parse TransferSolWithProgram data");
    let args = match TransferSolWithProgram::try_from_bytes(data) {
        Ok(parsed) => {
            msg!("✅ Successfully parsed transfer amount: {}", parsed.amount);
            parsed
        },
        Err(err) => {
            msg!("❌ Failed to parse instruction data");
            msg!("Error: {:?}", err);
            msg!("Expected format: TransferSolWithProgram struct with u64 amount");
            msg!("Got data bytes: {:?}", data);
            return Err(ProgramError::InvalidInstructionData);
        }
    };

    msg!("Executing direct transfer of {} lamports", args.amount);

    // Transfer lamports with logging
    let mut payer_lamports = payer_info.try_borrow_mut_lamports()?;
    let mut recipient_lamports = recipient_info.try_borrow_mut_lamports()?;

    let new_payer_balance = payer_lamports
        .checked_sub(args.amount)
        .ok_or_else(|| {
            msg!("❌ Insufficient funds for transfer");
            msg!("Current balance: {}", **payer_lamports);
            msg!("Attempted transfer: {}", args.amount);
            TransferError::InvalidAmount
        })?;

    let new_recipient_balance = recipient_lamports
        .checked_add(args.amount)
        .ok_or_else(|| {
            msg!("❌ Overflow in recipient balance");
            msg!("Current balance: {}", **recipient_lamports);
            msg!("Attempted transfer: {}", args.amount);
            TransferError::InvalidAmount
        })?;

    **payer_lamports = new_payer_balance;
    **recipient_lamports = new_recipient_balance;

    msg!("✅ Transfer completed successfully");
    msg!("New payer balance: {}", new_payer_balance);
    msg!("New recipient balance: {}", new_recipient_balance);

    Ok(())
}