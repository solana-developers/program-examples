use steel::*;

use crate::prelude::*;

pub fn create_address_info(signer: Pubkey, data: AddressInfoData) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(account_pda().0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: CreateAddressInfo { data }.to_bytes(),
    }
}
