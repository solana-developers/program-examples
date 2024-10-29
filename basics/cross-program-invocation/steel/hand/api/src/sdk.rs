use steel::*;

use crate::prelude::*;

pub fn pull_lever(
    power: Pubkey,
    name: String,
) -> Instruction {
    let mut name_bytes = [0u8; 32];
    name_bytes[..name.len()].copy_from_slice(name.as_bytes());

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(power, false),
            AccountMeta::new_readonly(lever_api::ID, false),
        ],
        data: PullLeverArgs {
            name_len: name.len() as u32,
            name: name_bytes,
        }.to_bytes(),
    }
}

