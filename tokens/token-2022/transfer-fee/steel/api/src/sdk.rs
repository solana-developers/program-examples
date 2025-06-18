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
