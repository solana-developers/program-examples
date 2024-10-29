use solana_program::{msg, program::invoke, rent::Rent, system_instruction};
use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [payer, new_account, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Determine the necessary minimum rent by calculating the account's size
    //
    let account_span = instruction_data.len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    msg!("Account span: {}", &account_span);
    msg!("Lamports required: {}", &lamports_required);
    
    
    // use `allocate_account`` steel helper for PDAs
    // allocate_account(target_account, system_program, payer, space, owner, seeds)
    //
    invoke(
        &system_instruction::create_account(
            payer.key,
            new_account.key,
            lamports_required,
            account_span as u64,
            &system_program::ID,
        ),
        &[payer.clone(), new_account.clone(), system_program.clone()],
    )?;

    msg!("Account created succesfully.");
    Ok(())
}

