use solana_program::msg;
use steel::*;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // The instruction data passed in
    msg!(
        "Hello, {}!",
        String::from_utf8(instruction_data.to_vec()).unwrap()
    );

    // Check the supplied program id is the same as our program ID
    if steel_hello_solana_api::ID.ne(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    };

    // Our Program ID
    msg!("Our program's Program ID: {}", &program_id);

    // The number of accounts passed in
    msg!("We have {} accounts", accounts.len());

    for (id, account) in accounts.iter().enumerate() {
        // The PublicKey of the account
        msg!("Account {} PublicKey: {}", id, account.key);
        // Do we expect a signature from this account?
        msg!("Account {} is signer?: {}", id, account.is_signer);
        // Will this account be modified in this instruction?
        msg!("Account {} is writable?: {}", id, account.is_writable);
    }

    Ok(())
}
