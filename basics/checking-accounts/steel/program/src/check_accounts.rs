use checking_accounts_api::prelude::*;
use steel::*;


pub fn process_check_accounts(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let [payer_info, account_to_create_info, account_to_change_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Check 1: Validate payer is a signer
    // This corresponds to Anchor's: payer: Signer<'info>
    payer_info
        .is_signer()
        .map_err(|_| ProgramError::MissingRequiredSignature)?;

    // Check 2: Validate account_to_create is mutable
    // This corresponds to Anchor's: #[account(mut)] account_to_create: UncheckedAccount<'info>
    account_to_create_info
        .is_writable()
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Check 3: Validate account_to_change is mutable and owned by our program
    // This corresponds to Anchor's: #[account(mut, owner = id())] account_to_change: UncheckedAccount<'info>
    account_to_change_info
        .is_writable()
        .map_err(|_| ProgramError::InvalidAccountData)?
        .has_owner(&ID)
        .map_err(|_| ValidationError::InvalidOwner)?;

    // Check 4: Validate system_program is the actual system program
    // This corresponds to Anchor's: system_program: Program<'info, System>
    system_program
        .is_program(&system_program::ID)
        .map_err(|_| ProgramError::IncorrectProgramId)?;

    // Optional: You can chain additional checks using Steel's validation traits
    // For example, checking if an account is empty:
    // account_to_create_info.is_empty()?;

    // Or checking if an account is executable:
    // system_program.is_executable()?;

    // Or checking PDA seeds:
    // account_info.has_seeds(&[b"seed"], &program_id)?;

    Ok(())
}
