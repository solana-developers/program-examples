use steel::*;

use crate::prelude::*;

/// Create an PDA and store a String in it
pub fn create_account(signer: Pubkey, user: CreateAccount) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(User::pda(signer).0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: user.to_bytes(),
    }
}

/// Creates an instruction to close the account,
/// in our case the PDA. The PDA address is derived from
/// the `payer` public key
pub fn close_account(signer: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(User::pda(signer).0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: CloseAccount.to_bytes(),
    }
}
