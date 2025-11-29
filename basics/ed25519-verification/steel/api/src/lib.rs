use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use steel::steel_api;

#[steel_api]
pub mod ed25519_custodial {
    use super::*;

    #[derive(Debug)]
    pub struct TransferAccounts {
        pub custodial_account: Pubkey,
        pub recipient: Pubkey,
        pub signer: Pubkey,
    }

    pub fn transfer(
        program_id: Pubkey,
        accounts: TransferAccounts,
        signature: [u8; 64],
        public_key: [u8; 32],
        message: Vec<u8>,
        amount: u64,
    ) -> Instruction {
        let accounts = vec![
            AccountMeta::new(accounts.custodial_account, false),
            AccountMeta::new(accounts.recipient, false),
            AccountMeta::new_readonly(accounts.signer, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ];

        let mut data = Vec::with_capacity(104 + message.len());
        data.extend_from_slice(&signature);
        data.extend_from_slice(&public_key);
        data.extend_from_slice(&amount.to_le_bytes());
        data.extend_from_slice(&message);

        Instruction {
            program_id,
            accounts,
            data,
        }
    }
} 