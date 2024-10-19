use steel::*;
use steel_api::prelude::*;

pub fn process_transfer_with_program(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = TransferWithProgram::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [payer, recipient] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // validate signer

    payer.is_signer()?;
    // system_program.is_program(&system_program::ID)?;

    **payer.try_borrow_mut_lamports()? -= amount;
    **recipient.try_borrow_mut_lamports()? += amount;

    Ok(())
}
