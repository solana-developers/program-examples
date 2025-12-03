use steel::*;

use crate::prelude::*;

pub fn initialize(signer: Pubkey, mint: Pubkey, rate: i16) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint, true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token_2022::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
        ],
        data: Initialize {
            rate: rate.to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn update_rate(signer: Pubkey, mint: Pubkey, rate: i16) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token_2022::ID, false),
        ],
        data: UpdateRate {
            rate: rate.to_le_bytes(),
        }
        .to_bytes(),
    }
}

