use steel::*;

use crate::prelude::*;

pub fn check_accounts(signer: Pubkey, account_to_create: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(account_to_create, false),
            AccountMeta::new(account_to_change_pda().0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: CheckAccounts {}.to_bytes(),
    }
}
