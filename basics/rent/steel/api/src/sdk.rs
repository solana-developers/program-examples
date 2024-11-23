use crate::prelude::*;
use steel::*;

pub fn create_system_account(
    payer: Pubkey,
    new_account: Pubkey,
    name: String,
    address: String,
) -> Instruction {
    let mut name_bytes = [0u8; 32];
    let mut address_bytes = [0u8; 64];

    name_bytes[..name.len()].copy_from_slice(name.as_bytes());
    address_bytes[..address.len()].copy_from_slice(address.as_bytes());

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(new_account, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: CreateSystemAccount {
            name: name_bytes,
            address: address_bytes,
        }
        .to_bytes(),
    }
}
