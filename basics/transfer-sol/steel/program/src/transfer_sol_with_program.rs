use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};
use steel::*;
use transfer_sol_api::prelude::*;

pub fn process_transfer_sol_with_program(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = TransferSolWithProgram::try_from_bytes(data)?;
    let amount: u64 = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, receiver_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;

    **signer_info.try_borrow_mut_lamports()? -= amount;
    **receiver_info.try_borrow_mut_lamports()? += amount;

    Ok(())
}
