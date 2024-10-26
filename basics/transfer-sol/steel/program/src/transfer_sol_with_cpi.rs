use steel::*;
use transfer_sol_api::prelude::*;

pub fn process_transfer_sol_with_cpi(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    let args = TransferSolWithCpi::try_from_bytes(data)?;
    let lamports = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [payer_info, recipient_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // check if the payer is signer
    payer_info.is_signer()?;

    // check recipient is SystemAccount
    if recipient_info.owner.ne(&system_program::ID) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    system_program.is_program(&system_program::ID)?;

    // Transfer lamports from payer to recipient by
    // invoke the system program to transfer lamports
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(payer_info.key, recipient_info.key, lamports),
        &[
            payer_info.clone(),
            recipient_info.clone(),
            system_program.clone(),
        ],
    )?;

    Ok(())
}
