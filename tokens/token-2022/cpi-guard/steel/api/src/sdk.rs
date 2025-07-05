use steel::*;

use crate::prelude::*;

pub fn cpi_burn(signer: Pubkey, mint: Pubkey, recipient_token_account: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint, false),
            AccountMeta::new(recipient_token_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token_2022::ID, false),
        ],
        data: CpiBurn {}.to_bytes(),
    }
}
