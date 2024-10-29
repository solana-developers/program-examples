use steel::*;

use crate::prelude::*;

pub fn initialize(
    payer: Pubkey,
    message_account: Pubkey,
    input: String,
) -> Instruction {
    let mut message_bytes = [0u8; 1024];
    message_bytes[..input.len()].copy_from_slice(input.as_bytes());

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(message_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Initialize {
            message_len: input.len() as u32,
            message: message_bytes,
        }.to_bytes(),
    }
}

pub fn update(
    payer: Pubkey,
    message_account: Pubkey,
    input: String,
) -> Instruction {
    let mut message_bytes = [0u8; 1024];
    message_bytes[..input.len()].copy_from_slice(input.as_bytes());

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(message_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Update {
            message_len: input.len() as u32,
            message: message_bytes,
        }.to_bytes(),
    }
}
