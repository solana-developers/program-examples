use steel::*;

use crate::prelude::*;

pub fn transfer_with_cpi(signer: Pubkey, recipient : Pubkey, amount: u64) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(counter_pda().0, false),
        ],
        data: TransferWithCPI {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn transfer_with_program(signer: Pubkey,recipient : Pubkey, amount: u64) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(counter_pda().0, false),
        ],
        data: TransferWithProgram {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

