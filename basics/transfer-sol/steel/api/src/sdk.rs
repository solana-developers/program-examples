use steel::*;

use crate::prelude::*;

pub fn with_cpi(payer: Pubkey, receiver: Pubkey, amount: u64) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(receiver, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: WithCpi {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn with_program(payer: Pubkey, receiver: Pubkey, amount: u64) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, false),
            AccountMeta::new(receiver, false),
        ],
        data: WithProgram {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}
