use steel::*;

use crate::prelude::*;

pub fn create_system_account(signer: Pubkey) -> Instruction {
    Instruction{
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(account_pda().0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: CreateSystemAccount { data }.to_bytes(),
    }
}

