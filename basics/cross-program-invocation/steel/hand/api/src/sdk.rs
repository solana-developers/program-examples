use steel::*;

use crate::prelude::*;

pub fn initialize(signer: Pubkey, power_account: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(power_account, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Initialize {}.to_bytes(),
    }
}

pub fn set_power_status(power_account: Pubkey, name: &str) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![AccountMeta::new(power_account, false)],
        data: SetPowerStatus {
            name: str_to_bytes(name),
        }
        .to_bytes(),
    }
}
