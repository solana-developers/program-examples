use steel::*;

use crate::prelude::*;

pub fn initialize_account(signer: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(new_account_pda().0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: InitializeAccount {}.to_bytes(),
    }
}
