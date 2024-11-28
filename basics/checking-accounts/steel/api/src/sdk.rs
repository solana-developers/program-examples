use steel::*;

use crate::prelude::*;

pub fn check_accounts(
    payer: Pubkey,
    account_to_create: Pubkey,
    account_to_change: Pubkey,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),  // payer must be signer
            AccountMeta::new(account_to_create, false),  // mutable but not signer
            AccountMeta::new(account_to_change, false),  // mutable but not signer
            AccountMeta::new_readonly(system_program::ID, false),  // system program
        ],
        data: CheckAccountsArgs {}.to_bytes(),
    }
}
