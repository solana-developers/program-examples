use steel::*;

use crate::prelude::*;

pub fn initialize(
    signer: Pubkey,
    mint: Pubkey,
    maximum_fee: u64,
    transfer_fee_basis_points: u16,
    decimals: u8,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint, true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token_2022::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
        ],
        data: Initialize {
            maximum_fee: maximum_fee.to_le_bytes(),
            transfer_fee_basis_points: transfer_fee_basis_points.to_le_bytes(),
            decimals,
        }
        .to_bytes(),
    }
}

pub fn transfer(
    amount: u64,
    signer: Pubkey,
    mint: Pubkey,
    source_token_account: Pubkey,
    destination_token_account: Pubkey,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(source_token_account, false),
            AccountMeta::new(destination_token_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token_2022::ID, false),
        ],
        data: Transfer {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn harvest(
    signer: Pubkey,
    mint: Pubkey,
    harvest_acc_1: Pubkey,
    harvest_acc_2: Pubkey,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint, false),
            AccountMeta::new(harvest_acc_1, false),
            AccountMeta::new(harvest_acc_2, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token_2022::ID, false),
        ],
        data: Harvest {}.to_bytes(),
    }
}

pub fn withdraw(signer: Pubkey, mint: Pubkey, destination: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint, false),
            AccountMeta::new(destination, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token_2022::ID, false),
        ],
        data: Withdraw {
            destination: destination.to_bytes(),
        }
        .to_bytes(),
    }
}

pub fn update_fee(
    signer: Pubkey,
    mint: Pubkey,
    maximum_fee: u64,
    transfer_fee_basis_points: u16,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token_2022::ID, false),
        ],
        data: UpdateFee {
            maximum_fee: maximum_fee.to_le_bytes(),
            transfer_fee_basis_points: transfer_fee_basis_points.to_le_bytes(),
        }
        .to_bytes(),
    }
}
