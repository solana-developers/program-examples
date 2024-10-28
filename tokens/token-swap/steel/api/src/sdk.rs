use steel::*;
use sysvar::rent::{self};

use crate::prelude::*;

pub fn create_amm(signer: Pubkey, amm: Pubkey, admin: Pubkey, id: Pubkey, fee: u16) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(amm, false),
            AccountMeta::new(admin, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: CreateAmm {
            id: id.to_bytes(),
            fee: fee.to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn create_pool(
    signer: Pubkey,
    amm: Pubkey,
    pool: Pubkey,
    pool_authority: Pubkey,
    mint_liquidity: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    pool_token_account_a: Pubkey,
    pool_token_account_b: Pubkey,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(amm, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(pool_authority, false),
            AccountMeta::new(mint_liquidity, false),
            AccountMeta::new(mint_a, false),
            AccountMeta::new(mint_b, false),
            AccountMeta::new(pool_token_account_a, false),
            AccountMeta::new(pool_token_account_b, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(rent::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: CreatePool {}.to_bytes(),
    }
}

pub fn deposit_liquidity(
    signer: Pubkey,
    pool: Pubkey,
    pool_authority: Pubkey,
    depositor: Pubkey,
    mint_liquidity: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    pool_token_account_a: Pubkey,
    pool_token_account_b: Pubkey,
    depositor_token_account_liquidity: Pubkey,
    depositor_token_account_a: Pubkey,
    depositor_token_account_b: Pubkey,
    amount_a: u64,
    amount_b: u64,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(depositor, true),
            AccountMeta::new(pool, false),
            AccountMeta::new(pool_authority, false),
            AccountMeta::new(mint_liquidity, false),
            AccountMeta::new(mint_a, false),
            AccountMeta::new(mint_b, false),
            AccountMeta::new(pool_token_account_a, false),
            AccountMeta::new(pool_token_account_b, false),
            AccountMeta::new(depositor_token_account_liquidity, false),
            AccountMeta::new(depositor_token_account_a, false),
            AccountMeta::new(depositor_token_account_b, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: DepositLiquidity {
            amount_a: amount_a.to_le_bytes(),
            amount_b: amount_b.to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn swap_exact_tokens_for_tokens(
    signer: Pubkey,
    amm: Pubkey,
    pool: Pubkey,
    pool_authority: Pubkey,
    trader: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    pool_token_account_a: Pubkey,
    pool_token_account_b: Pubkey,
    trader_token_account_a: Pubkey,
    trader_token_account_b: Pubkey,
    swap_a: u8,
    input_amount: u64,
    min_output_amount: u64,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(trader, true),
            AccountMeta::new(amm, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(pool_authority, false),
            AccountMeta::new(mint_a, false),
            AccountMeta::new(mint_b, false),
            AccountMeta::new(pool_token_account_a, false),
            AccountMeta::new(pool_token_account_b, false),
            AccountMeta::new(trader_token_account_a, false),
            AccountMeta::new(trader_token_account_b, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: SwapExactTokens {
            swap_a,
            input_amount: input_amount.to_le_bytes(),
            min_output_amount: min_output_amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn withdraw_liquidity(
    signer: Pubkey,
    depositor: Pubkey,
    amm: Pubkey,
    pool: Pubkey,
    pool_authority: Pubkey,
    mint_liquidity: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    pool_token_account_a: Pubkey,
    pool_token_account_b: Pubkey,
    depositor_token_account_a: Pubkey,
    depositor_token_account_b: Pubkey,
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(depositor, true),
            AccountMeta::new(amm, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(pool_authority, false),
            AccountMeta::new(mint_liquidity, false),
            AccountMeta::new(mint_a, false),
            AccountMeta::new(mint_b, false),
            AccountMeta::new(pool_token_account_a, false),
            AccountMeta::new(pool_token_account_b, false),
            AccountMeta::new(depositor_token_account_a, false),
            AccountMeta::new(depositor_token_account_b, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: WithdrawLiquidity {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}
