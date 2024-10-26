use solana_program::msg;
use steel::*;
use transfer_sol_api::prelude::*;

pub fn process_with_cpi(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    msg!("Processing with cpi");

    // Parse args
    let args = WithProgram::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts
    let [signer_info, receiver_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?.is_writable()?;
    receiver_info.is_writable()?;
    system_program.is_program(&system_program::ID)?;

    receiver_info.collect(amount, signer_info)?;

    Ok(())
}
