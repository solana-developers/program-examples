use steel::*;

use crate::prelude::*;

pub fn initialize(
    user: Pubkey,
    power: Pubkey,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(power, false),
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: InitializeArgs {}.to_bytes(),
    }
}

pub fn switch_power(
    power: Pubkey,
    name: String,
) -> Instruction {
    let mut name_bytes = [0u8; 32];
    name_bytes[..name.len()].copy_from_slice(name.as_bytes());

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(power, false),
        ],
        data: SwitchPowerArgs {
            name_len: name.len() as u32,
            name: name_bytes,
        }.to_bytes(),
    }
}