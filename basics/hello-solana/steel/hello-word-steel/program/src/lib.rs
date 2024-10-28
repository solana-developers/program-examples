use steel::*;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct HelloWorld {
    pub message: [u8; 32], // Fixed-size array to hold the message
}


/// Initialize the HelloWorld account with "Hello World!" message
pub fn initialize(program_id: &Pubkey, accounts: &[AccountInfo], _input: &[u8]) -> ProgramResult {
    // Ensure there is at least one account
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    // Access the HelloWorld account directly from the `accounts` array
    let hello_world_account = &accounts[0];

    // Verify that the account owner matches the program ID
    if hello_world_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Set the initial message
    let message = b"Hello World!";
    let mut account_data = hello_world_account.try_borrow_mut_data()?;
    let hello_world: &mut HelloWorld = bytemuck::from_bytes_mut(&mut account_data);
    hello_world.message[..message.len()].copy_from_slice(message);

    msg!("Initialized HelloWorld account with message: Hello World!");
    Ok(())
}

/// Update the HelloWorld account message
pub fn update_message(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let hello_world_account = &accounts[0];

    if hello_world_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let new_message = &input[..input.len().min(32)];
    let mut account_data = hello_world_account.try_borrow_mut_data()?;
    let hello_world: &mut HelloWorld = bytemuck::from_bytes_mut(&mut account_data);
    hello_world.message[..new_message.len()].copy_from_slice(new_message);

    msg!("Updated HelloWorld account message.");
    Ok(())
}
