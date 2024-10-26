use steel::*;
use transfer_sol_api::prelude::*;
pub fn process_transfer_sol_with_program(
    accounts: &[AccountInfo<'_>],
    data: &[u8],
) -> ProgramResult {
    let args = TransferSolWithProgram::try_from_bytes(data)?;
    let lamports = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [payer_info, recipient_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // // check recipient is SystemAccount
    if recipient_info.owner.ne(&system_program::ID) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Transfer lamports from payer to recipient
    payer_info.send(lamports, recipient_info);
    Ok(())
}
