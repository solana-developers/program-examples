use crate::prelude::*;
use steel::*;

pub fn pull_level(power: Pubkey, name: &str) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(power, false),
            AccountMeta::new_readonly(lever_api::ID, false),
        ],
        data: PullLever {
            name: name.as_bytes().try_into().unwrap(),
        }
        .to_bytes(),
    }
}
