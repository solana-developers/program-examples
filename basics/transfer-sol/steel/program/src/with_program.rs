use solana_program::msg;
use steel::*;
use transfer_sol_api::prelude::*;

pub fn proces_with_program(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    msg!("Processing with program");

    // Parse args
    let args = WithProgram::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts
    let [payer_info, receiver_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    payer_info.is_writable()?;
    receiver_info.is_writable()?;

    **payer_info.try_borrow_mut_lamports()? -= amount;
    **receiver_info.try_borrow_mut_lamports()? += amount;

    // signer_info.send(amount, receiver_info);

    //     **payer.try_borrow_mut_lamports()? -= amount;
    //     **recipient.try_borrow_mut_lamports()? += amount;

    Ok(())
}
