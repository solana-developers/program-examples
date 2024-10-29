use solana_program::{msg, native_token::LAMPORTS_PER_SOL, program::invoke, system_instruction};
use steel::*;

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

    // use `create_account` or `allocate_account` steel helper for creating 
    // account to be owned by programs other than the system program.
    //
    invoke(
        &system_instruction::create_account(
            payer.key,
            new_account.key,
            LAMPORTS_PER_SOL,
            0,
            &system_program::ID,
        ),
        &[payer.clone(), new_account.clone(), system_program.clone()],
    )?;

    msg!("Account created succesfully.");

    Ok(())
}
