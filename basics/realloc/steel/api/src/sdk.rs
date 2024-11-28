use steel::*;
use crate::prelude::*;

pub fn initialize(
    payer: Pubkey,
    message_account: Pubkey,
    message: String,
) -> Instruction {
    let mut message_bytes = [0u8; 1024];
    message_bytes[..message.len()].copy_from_slice(message.as_bytes());

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(message_account, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Initialize {
            message: message_bytes,
            len: (message.len() as u32).to_le_bytes(),
        }.to_bytes(),
    }
}

pub fn update(
    payer: Pubkey,
    message_account: Pubkey,
    message: String,
) -> Instruction {
    let mut message_bytes = [0u8; 1024];
    message_bytes[..message.len()].copy_from_slice(message.as_bytes());

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(message_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Update {
            message: message_bytes,
            len: (message.len() as u32).to_le_bytes(),
        }.to_bytes(),
    }
}