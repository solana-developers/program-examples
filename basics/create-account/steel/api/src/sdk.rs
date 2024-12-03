use steel::*;

use crate::prelude::*;

pub fn create_system_account(payer: Pubkey, new_account: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(new_account, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: CreateSystemAccount {}.to_bytes(),
    }
}
