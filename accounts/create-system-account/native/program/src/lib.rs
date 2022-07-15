use solana_program::{
    account_info::{AccountInfo, next_account_info}, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    native_token::LAMPORTS_PER_SOL,
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
    system_program,
};


entrypoint!(process_instruction);


fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let new_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    
    msg!("Program invoked. Creating a system account...");
    msg!("  New public key will be: {}", &new_account.key.to_string());
    
    invoke(
        &system_instruction::create_account(
            &payer.key,             // From pubkey
            &new_account.key,       // To pubkey
            1 * LAMPORTS_PER_SOL,   // Lamports (1 SOL)
            0,                      // Space
            &system_program::ID,    // Owner
        ),
        &[
            payer.clone(), new_account.clone(), system_program.clone()  // Accounts involved
        ]
    )?;

    msg!("Account created succesfully.");
    Ok(())
}