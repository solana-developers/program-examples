use steel::*;

use crate::prelude::*;

pub fn create_user(user: Pubkey, name: &str) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(user_state_pda(user).0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: CreateUser {
            name: name.as_bytes().try_into().unwrap(),
        }
        .to_bytes(),
    }
}

pub fn close_user(user: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(user_state_pda(user).0, false),
        ],
        data: CloseUser {}.to_bytes(),
    }
}
