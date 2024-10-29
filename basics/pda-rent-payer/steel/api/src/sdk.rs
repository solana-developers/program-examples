use steel::*;

use crate::prelude::*;

pub fn init_rent_vault(signer_info: Pubkey, system_program: Pubkey, data: &[u8]) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer_info, true),
            AccountMeta::new(rent_vault_pda().0, false),
            AccountMeta::new_readonly(system_program, false),
        ],
        data: InitializeRentVault {
            amount: u64::from_be_bytes(data[..8].try_into().unwrap()),
        }
        .to_bytes(),
    }
}

pub fn create_new_account(rent_vault: Pubkey, new_account: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(rent_vault, false),
            AccountMeta::new(new_account, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: CreateNewAccount {}.to_bytes(),
    }
}
