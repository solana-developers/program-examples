use crate::{constants::MINIMUM_LIQUIDITY, errors::AmmError, state::Pool};
use borsh::{BorshDeserialize, BorshSerialize};
use fixed::types::I64F64;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::{
    instruction::{burn, transfer},
    state::{Account as TokenAccount, Mint},
};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct WithdrawLiquidityArgs {
    amount: u64,
}

pub fn process_withdraw_liquidity(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: WithdrawLiquidityArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let pool = next_account_info(accounts_iter)?;
    let pool_authority = next_account_info(accounts_iter)?;
    let depositor = next_account_info(accounts_iter)?;
    let mint_liquidity = next_account_info(accounts_iter)?;
    let mint_a = next_account_info(accounts_iter)?;
    let mint_b = next_account_info(accounts_iter)?;
    let pool_account_a = next_account_info(accounts_iter)?;
    let pool_account_b = next_account_info(accounts_iter)?;
    let depositor_account_liquidity = next_account_info(accounts_iter)?;
    let depositor_account_a = next_account_info(accounts_iter)?;
    let depositor_account_b = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // Check that the pool corresponds to the target mints
    let pool_data = Pool::try_from_slice(&pool.data.borrow())?;
    if &pool_data.mint_a != mint_a.key || &pool_data.mint_b != mint_b.key {
        return Err(ProgramError::InvalidAccountData);
    }

    // Verify pool_authority PDA
    let pool_authority_seeds = &[
        Pool::AUTHORITY_PREFIX.as_ref(),
        pool_data.amm.as_ref(),
        mint_a.key.as_ref(),
        mint_b.key.as_ref(),
    ];
    let (pool_authority_pda, pool_authority_bump) =
        Pubkey::find_program_address(pool_authority_seeds, program_id);
    if pool_authority.key != &pool_authority_pda {
        return Err(AmmError::InvalidAuthority.into());
    }

    // Transfer tokens from the pool
    let pool_token_account_data_a = TokenAccount::unpack(&pool_account_a.data.borrow())?;
    let pool_token_account_data_b = TokenAccount::unpack(&pool_account_b.data.borrow())?;
    let mint_liquidity_data = Mint::unpack(&mint_liquidity.data.borrow())?;
    let amount_a = I64F64::from_num(args.amount)
        .checked_mul(I64F64::from_num(pool_token_account_data_a.amount))
        .unwrap()
        .checked_div(I64F64::from_num(
            mint_liquidity_data.supply + MINIMUM_LIQUIDITY,
        ))
        .unwrap()
        .floor()
        .to_num::<u64>();

    invoke_signed(
        &transfer(
            token_program.key,
            pool_account_a.key,
            depositor_account_a.key,
            pool_authority.key,
            &[],
            amount_a,
        )?,
        &[
            pool_account_a.clone(),
            depositor_account_a.clone(),
            pool_authority.clone(),
            token_program.clone(),
        ],
        &[&[
            Pool::AUTHORITY_PREFIX.as_ref(),
            pool_data.amm.as_ref(),
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
            &[pool_authority_bump],
        ]],
    )?;

    let amount_b = I64F64::from_num(args.amount)
        .checked_mul(I64F64::from_num(pool_token_account_data_b.amount))
        .unwrap()
        .checked_div(I64F64::from_num(
            mint_liquidity_data.supply + MINIMUM_LIQUIDITY,
        ))
        .unwrap()
        .floor()
        .to_num::<u64>();

    invoke_signed(
        &transfer(
            token_program.key,
            pool_account_b.key,
            depositor_account_b.key,
            pool_authority.key,
            &[],
            amount_b,
        )?,
        &[
            pool_account_b.clone(),
            depositor_account_b.clone(),
            pool_authority.clone(),
            token_program.clone(),
        ],
        &[&[
            Pool::AUTHORITY_PREFIX.as_ref(),
            pool_data.amm.as_ref(),
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
            &[pool_authority_bump],
        ]],
    )?;

    // Burn the liquidity tokens
    // It will fail if the amount is invalid
    invoke(
        &burn(
            token_program.key,
            depositor_account_liquidity.key,
            mint_liquidity.key,
            depositor.key,
            &[],
            args.amount,
        )?,
        &[
            depositor.clone(),
            depositor_account_liquidity.clone(),
            mint_liquidity.clone(),
            token_program.clone(),
        ],
    )?;

    Ok(())
}
