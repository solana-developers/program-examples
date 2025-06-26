use steel::*;

use crate::prelude::*;

pub fn initialize(signer: Pubkey, mint: Pubkey, token_account: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint, false),
            AccountMeta::new(token_account, true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token_2022::ID, false),
        ],
        data: Initialize {}.to_bytes(),
    }
}
