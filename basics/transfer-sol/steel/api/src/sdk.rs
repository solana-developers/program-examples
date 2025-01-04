use steel::*;

use crate::prelude::*;

pub fn transfer_sol_with_cpi(signer: Pubkey, receiver: Pubkey, amount: u64) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(receiver, false),
            AccountMeta::new(system_program::ID, false),
        ],
        data: TransferSolWithCpi {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn transfer_sol_with_program(signer: Pubkey, receiver: Pubkey, amount: u64) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(receiver, false),
        ],
        data: TransferSolWithProgram {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}
