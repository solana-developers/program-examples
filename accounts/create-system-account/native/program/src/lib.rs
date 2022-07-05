use solana_program::{
    account_info::AccountInfo, 
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
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    
    msg!("Program invoked. Creating a system account...");
    msg!("New public key will be: {}", &new_pubkey);
    
    invoke(
        &system_instruction::create_account(
            &payer,                 // From pubkey
            &new_pubkey,            // To pubkey
            LAMPORTS_PER_SOL,       // Lamports
            32,                     // Space
            &system_program::ID,    // Owner
        ),
        &[payer]                // Signers
    )?;

    msg!("Account created succesfully.");
    Ok(())
}