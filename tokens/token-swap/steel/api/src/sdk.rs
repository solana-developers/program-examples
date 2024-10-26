use steel::*;

use crate::prelude::*;

pub fn create_amm(payer: Pubkey, admin: Pubkey, id: Pubkey, fee: u16) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(admin, false),
            AccountMeta::new(amm_pda(id).0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: CreateAmm {
            id,
            fee: fee.to_be_bytes(),
        }
        .to_bytes(),
    }
}

pub fn create_pool(payer: Pubkey, amm: Pubkey, mint_a: Pubkey, mint_b: Pubkey) -> Instruction {
    let pool_authority = pool_authority_pda(amm, mint_a, mint_b).0;
    let (pool_account_a, _) = Pubkey::find_program_address(
        &[
            &pool_authority.to_bytes(),
            &spl_token::ID.to_bytes(),
            &mint_a.to_bytes(),
        ],
        &spl_token::ID,
    );

    let (pool_account_b, _) = Pubkey::find_program_address(
        &[
            &pool_authority.to_bytes(),
            &spl_token::ID.to_bytes(),
            &mint_b.to_bytes(),
        ],
        &spl_token::ID,
    );
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(amm, false),
            AccountMeta::new(pool_pda(amm, mint_a, mint_b).0, false),
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(mint_liquidity_pda(amm, mint_a, mint_b).0, false),
            AccountMeta::new(pool_account_a, false),
            AccountMeta::new(pool_account_b, false),
            AccountMeta::new(mint_a, false),
            AccountMeta::new(mint_b, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
        ],
        data: CreatePool {}.to_bytes(),
    }
}
