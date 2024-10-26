use steel::*;

use crate::prelude::*;

pub fn transfer_sol_with_cpi(payer: Pubkey, recipient: Pubkey, amount: u64) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(recipient, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: TransferSolWithCpi {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn transfer_sol_with_program(payer: Pubkey, recipient: Pubkey, amount: u64) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, false),
            AccountMeta::new(recipient, false),
        ],
        data: TransferSolWithProgram {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}
