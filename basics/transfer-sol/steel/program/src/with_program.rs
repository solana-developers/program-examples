use solana_program::msg;
use steel::*;
use transfer_sol_api::prelude::*;

pub fn proces_with_program(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    msg!("Processing with program");

    // Parse args
    let args = WithProgram::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts
    let [signer_info, receiver_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?.is_writable()?;
    receiver_info.is_writable()?;

    receiver_info.collect(amount, signer_info)?;

    // invoke(
    //     &system_instruction::transfer(payer.key, recipient.key, amount),
    //     &[payer.clone(), recipient.clone(), system_program.clone()],
    // )?;
    Ok(())
}
