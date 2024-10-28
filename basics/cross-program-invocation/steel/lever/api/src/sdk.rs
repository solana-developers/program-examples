use steel::*;

use crate::prelude::*;

pub fn initialize(user: Pubkey, power: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(power, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Initialize {}.to_bytes(),
    }
}

pub fn switch_power(power: Pubkey, name: &str) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![AccountMeta::new(power, false)],
        data: SwitchPower {
            name: name.as_bytes().try_into().unwrap(),
        }
        .to_bytes(),
    }
}
