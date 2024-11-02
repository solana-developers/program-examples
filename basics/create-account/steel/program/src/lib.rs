use solana_program::{msg, program::invoke, system_instruction};
use steel::*;
use sysvar::rent::Rent;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    let [payer, new_account, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    msg!("Program invoked. Creating a system account...");
    msg!("  New public key will be: {}", &new_account.key.to_string());

    // get minimum lamports required for 0 data space
    //
    let lamports = Rent::get()?.minimum_balance(0);

    // use `create_account` or `allocate_account` steel helper for creating
    // pda accounts.
    //
    invoke(
        &system_instruction::create_account(
            payer.key,
            new_account.key,
            lamports,            // send lmaports
            0,                   // space
            &system_program::ID, // owner program
        ),
        &[payer.clone(), new_account.clone(), system_program.clone()],
    )?;

    msg!("Account created succesfully.");

    Ok(())
}
