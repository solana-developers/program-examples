#[derive(Debug)]
#[repr(C)]
pub struct AddressInfo {
    pub name: [u8; 32],
    pub house_number: u8,
    pub street: [u8; 64],
    pub city: [u8; 32],
}

impl AddressInfo {
    pub fn new(name: &str, house_number: u8, street: &str, city: &str) -> Self {
        let mut addr_info = AddressInfo {
            name: [0; 32],
            house_number,
            street: [0; 64],
            city: [0; 32],
        };

        // Copy strings into fixed-size arrays, truncating if necessary
        let name_bytes = name.as_bytes();
        let copy_len = name_bytes.len().min(32);
        addr_info.name[..copy_len].copy_from_slice(&name_bytes[..copy_len]);

        let street_bytes = street.as_bytes();
        let copy_len = street_bytes.len().min(64);
        addr_info.street[..copy_len].copy_from_slice(&street_bytes[..copy_len]);

        let city_bytes = city.as_bytes();
        let copy_len = city_bytes.len().min(32);
        addr_info.city[..copy_len].copy_from_slice(&city_bytes[..copy_len]);

        addr_info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use {
        mollusk_svm::{program::keyed_account_for_system_program, Mollusk},
        solana_sdk::{
            account::Account,
            instruction::{AccountMeta, Instruction},
            pubkey,
            pubkey::Pubkey,
        },
    };

    #[test]
    fn test() {
        let program_id = pubkey!("51ZnFwUuX1mW5ijGY1MrHimfqE1voiKCfTKnDXuX71Qw");
        let (system_program, system_program_data) = keyed_account_for_system_program();
        let address_info_account = Pubkey::new_unique();
        let payer = Pubkey::new_unique();

        let accounts = vec![
            AccountMeta::new(address_info_account, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program, false),
        ];

        let mut data = vec![0];
        let address_info_data = AddressInfo::new("perelyn", 0, "solana", "solana");

        // Convert struct to bytes using unsafe transmute since we have #[repr(C)]
        let address_info_bytes = unsafe {
            std::slice::from_raw_parts(
                &address_info_data as *const AddressInfo as *const u8,
                std::mem::size_of::<AddressInfo>(),
            )
        };
        data.extend_from_slice(address_info_bytes);

        let instruction = Instruction::new_with_bytes(program_id, &data, accounts);

        let base_lamports = 100_000_000u64;

        let accounts = vec![
            (address_info_account, Account::new(0, 0, &Pubkey::default())),
            (payer, Account::new(base_lamports, 0, &Pubkey::default())),
            (system_program, system_program_data),
        ];

        let mollusk = Mollusk::new(&program_id, "account_data_program");

        // Execute the instruction and get the result.
        let result = mollusk.process_instruction(&instruction, &accounts);
        dbg!(result.compute_units_consumed);
    }
}
