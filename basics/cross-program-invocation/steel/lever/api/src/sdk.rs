use steel::*;

use crate::prelude::*;

pub fn initialize(user: Pubkey, power_account: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(power_account, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Initialize {}.to_bytes(),
    }
}

pub fn switch_power(power_account: Pubkey, name: &str) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![AccountMeta::new(power_account, false)],
        data: SetPowerStatus {
            name: str_to_bytes(name),
        }
        .to_bytes(),
    }
}
