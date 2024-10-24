use steel::*;

use crate::state::*;
use crate::instruction::*;

pub fn initialize(signer: Pubkey, name: &str, house_number: u8, city: &str, street: &str) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(account_pda().0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: InitializeAccount {
            name: name.as_bytes().try_into().unwrap(),
            house_number: house_number,
            city: city.as_bytes().try_into().unwrap(),
            street: street.as_bytes().try_into().unwrap(),
        }.to_bytes()
    }
}
