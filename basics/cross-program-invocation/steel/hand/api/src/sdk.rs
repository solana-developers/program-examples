use lever_api::prelude::*;
use steel::*;

use crate::prelude::*;

pub fn pull_lever(power_account: Pubkey, name: &str) -> Instruction {
    // pub fn pull_lever(power_account: Pubkey, lever_program: Pubkey, name: &str) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(power_account, false),
            // AccountMeta::new(lever_program, false),
            AccountMeta::new_readonly(lever_api::ID, false),
        ],
        data: PullLever {
            name: str_to_bytes(name),
        }
        .to_bytes(),
    }
}
