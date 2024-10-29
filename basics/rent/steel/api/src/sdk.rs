use steel::*;

use crate::prelude::*;

pub fn create_system_account(
    payer: Pubkey,
    new_account: Pubkey,
    name: String,
    address: String,
) -> Result<Instruction, ProgramError> {
    // Validate inputs
    if name.len() > STRING_MAX_SIZE || address.len() > STRING_MAX_SIZE {
        return Err(RentError::StringTooLong.into());
    }

    // Convert strings to fixed-size arrays
    let mut name_bytes = [0u8; 32];
    let mut address_bytes = [0u8; 32];

    name_bytes[..name.len()].copy_from_slice(name.as_bytes());
    address_bytes[..address.len()].copy_from_slice(address.as_bytes());

    Ok(Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(new_account, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: CreateSystemAccountArgs {
            name_len: name.len() as u32,
            name: name_bytes,
            address_len: address.len() as u32,
            address: address_bytes,
        }.to_bytes(),
    })
}