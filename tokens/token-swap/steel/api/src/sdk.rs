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
            fee: fee.to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn create_pool(payer: Pubkey, amm: Pubkey, mint_a: Pubkey, mint_b: Pubkey) -> Instruction {
    let pool_authority = pool_authority_pda(amm, mint_a, mint_b).0;
    let (pool_account_a, _) = Pubkey::find_program_address(
        &[
            pool_authority.as_ref(),
            spl_token::ID.as_ref(),
            mint_a.as_ref(),
        ],
        &spl_token::ID,
    );

    let (pool_account_b, _) = Pubkey::find_program_address(
        &[
            pool_authority.as_ref(),
            spl_token::ID.as_ref(),
            mint_b.as_ref(),
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
            AccountMeta::new(mint_a, false),
            AccountMeta::new(mint_b, false),
            AccountMeta::new(pool_account_a, false),
            AccountMeta::new(pool_account_b, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
        ],
        data: CreatePool {}.to_bytes(),
    }
}

pub fn deposit_liquidity(
    payer: Pubkey,
    depositor: Pubkey,
    pool: Pubkey,
    pool_authority: Pubkey,
    amm: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    amount_a: u64,
    amount_b: u64,
) -> Instruction {
    let (pool_account_a, _) = Pubkey::find_program_address(
        &[
            pool_authority.as_ref(),
            spl_token::ID.as_ref(),
            mint_a.as_ref(),
        ],
        &spl_token::ID,
    );

    let (pool_account_b, _) = Pubkey::find_program_address(
        &[
            pool_authority.as_ref(),
            spl_token::ID.as_ref(),
            mint_b.as_ref(),
        ],
        &spl_token::ID,
    );

    let mint_liquidity = mint_liquidity_pda(amm, mint_a, mint_b).0;

    let (depositor_account_liquidity, _) = Pubkey::find_program_address(
        &[
            depositor.as_ref(),
            spl_token::ID.as_ref(),
            mint_liquidity.as_ref(),
        ],
        &spl_token::ID,
    );

    let (depositor_account_a, _) = Pubkey::find_program_address(
        &[depositor.as_ref(), spl_token::ID.as_ref(), mint_a.as_ref()],
        &spl_token::ID,
    );

    let (depositor_account_b, _) = Pubkey::find_program_address(
        &[depositor.as_ref(), spl_token::ID.as_ref(), mint_b.as_ref()],
        &spl_token::ID,
    );
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(depositor, true),
            AccountMeta::new_readonly(pool, false),
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(mint_liquidity, false),
            AccountMeta::new(mint_a, false),
            AccountMeta::new(mint_b, false),
            AccountMeta::new(pool_account_a, false),
            AccountMeta::new(pool_account_b, false),
            AccountMeta::new(depositor_account_liquidity, false),
            AccountMeta::new(depositor_account_a, false),
            AccountMeta::new(depositor_account_b, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM_ID, false),
        ],
        data: DepositLiquidity {
            amount_a: amount_a.to_le_bytes(),
            amount_b: amount_b.to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn withdraw_liquidity(
    payer: Pubkey,
    depositor: Pubkey,
    pool: Pubkey,
    pool_authority: Pubkey,
    amm: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    amount: u64,
) -> Instruction {
    let (pool_account_a, _) = Pubkey::find_program_address(
        &[
            pool_authority.as_ref(),
            spl_token::ID.as_ref(),
            mint_a.as_ref(),
        ],
        &spl_token::ID,
    );

    let (pool_account_b, _) = Pubkey::find_program_address(
        &[
            pool_authority.as_ref(),
            spl_token::ID.as_ref(),
            mint_b.as_ref(),
        ],
        &spl_token::ID,
    );

    let mint_liquidity = mint_liquidity_pda(amm, mint_a, mint_b).0;

    let (depositor_account_liquidity, _) = Pubkey::find_program_address(
        &[
            depositor.as_ref(),
            spl_token::ID.as_ref(),
            mint_liquidity.as_ref(),
        ],
        &spl_token::ID,
    );

    let (depositor_account_a, _) = Pubkey::find_program_address(
        &[depositor.as_ref(), spl_token::ID.as_ref(), mint_a.as_ref()],
        &spl_token::ID,
    );

    let (depositor_account_b, _) = Pubkey::find_program_address(
        &[depositor.as_ref(), spl_token::ID.as_ref(), mint_b.as_ref()],
        &spl_token::ID,
    );
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(depositor, true),
            AccountMeta::new_readonly(pool, false),
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(mint_liquidity, false),
            AccountMeta::new(mint_a, false),
            AccountMeta::new(mint_b, false),
            AccountMeta::new(pool_account_a, false),
            AccountMeta::new(pool_account_b, false),
            AccountMeta::new(depositor_account_liquidity, false),
            AccountMeta::new(depositor_account_a, false),
            AccountMeta::new(depositor_account_b, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM_ID, false),
        ],
        data: WithdrawLiquidity {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn swap(
    payer: Pubkey,
    trader: Pubkey,
    pool: Pubkey,
    pool_authority: Pubkey,
    amm: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    swap_a: bool,
    input_amount: u64,
    min_output_amount: u64,
) -> Instruction {
    let (pool_account_a, _) = Pubkey::find_program_address(
        &[
            pool_authority.as_ref(),
            spl_token::ID.as_ref(),
            mint_a.as_ref(),
        ],
        &spl_token::ID,
    );

    let (pool_account_b, _) = Pubkey::find_program_address(
        &[
            pool_authority.as_ref(),
            spl_token::ID.as_ref(),
            mint_b.as_ref(),
        ],
        &spl_token::ID,
    );

    let (trader_account_a, _) = Pubkey::find_program_address(
        &[trader.as_ref(), spl_token::ID.as_ref(), mint_a.as_ref()],
        &spl_token::ID,
    );

    let (trader_account_b, _) = Pubkey::find_program_address(
        &[trader.as_ref(), spl_token::ID.as_ref(), mint_b.as_ref()],
        &spl_token::ID,
    );
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(trader, true),
            AccountMeta::new_readonly(amm, false),
            AccountMeta::new_readonly(pool, false),
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(mint_a, false),
            AccountMeta::new(mint_b, false),
            AccountMeta::new(pool_account_a, false),
            AccountMeta::new(pool_account_b, false),
            AccountMeta::new(trader_account_a, false),
            AccountMeta::new(trader_account_b, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM_ID, false),
        ],
        data: Swap {
            swap_a: swap_a as u8,
            input_amount: input_amount.to_le_bytes(),
            min_output_amount: min_output_amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}
