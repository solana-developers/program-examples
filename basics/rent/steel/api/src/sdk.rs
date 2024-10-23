use steel::*;

use crate::state::*;
use crate::instruction::*;

pub fn initialize(signer: Pubkey, name: &str, address: &str) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(account_pda().0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: InitializeAccount {
            name: name.as_bytes().try_into().unwrap(),
            address: address.as_bytes().try_into().unwrap(),
        }.to_bytes()
    }
}
