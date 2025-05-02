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
    instruction::{mint_to, transfer},
    state::Account as TokenAccount,
};

use crate::{constants::MINIMUM_LIQUIDITY, errors::AmmError, state::Pool};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct DepositLiquidityArgs {
    amount_a: u64,
    amount_b: u64,
}

pub fn process_deposit_liquidity(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: DepositLiquidityArgs,
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

    // If the user specified amounts greater than held, use the total amounts they do have
    let depositor_token_account_data_a = TokenAccount::unpack(&depositor_account_a.data.borrow())?;
    let depositor_token_account_data_b = TokenAccount::unpack(&depositor_account_b.data.borrow())?;
    let mut amount_a = if args.amount_a > depositor_token_account_data_a.amount {
        depositor_token_account_data_a.amount
    } else {
        args.amount_a
    };
    let mut amount_b = if args.amount_b > depositor_token_account_data_b.amount {
        depositor_token_account_data_b.amount
    } else {
        args.amount_b
    };

    // Make sure they are provided in the same proportion as existing liquidity
    let pool_token_account_data_a = TokenAccount::unpack(&pool_account_a.data.borrow())?;
    let pool_token_account_data_b = TokenAccount::unpack(&pool_account_b.data.borrow())?;
    // Defining pool creation like this allows attackers to frontun pool creation with bad ratios
    let pool_creation =
        pool_token_account_data_a.amount == 0 && pool_token_account_data_b.amount == 0;
    (amount_a, amount_b) = if pool_creation {
        // Add as is if there is no liquidity
        (amount_a, amount_b)
    } else {
        let ratio = I64F64::from_num(pool_token_account_data_a.amount)
            .checked_div(I64F64::from_num(pool_token_account_data_b.amount))
            .unwrap();
        if pool_token_account_data_a.amount > pool_token_account_data_b.amount {
            (
                I64F64::from_num(amount_b)
                    .checked_mul(ratio)
                    .unwrap()
                    .to_num::<u64>(),
                amount_b,
            )
        } else {
            (
                amount_a,
                I64F64::from_num(amount_a)
                    .checked_div(ratio)
                    .unwrap()
                    .to_num::<u64>(),
            )
        }
    };

    // Computing the amount of liquidity about to be deposited
    let mut liquidity_amount = I64F64::from_num(amount_a)
        .checked_mul(I64F64::from_num(amount_b))
        .unwrap()
        .sqrt()
        .to_num::<u64>();

    // Lock some minimum liquidity in the first deposit
    if pool_creation {
        if liquidity_amount < MINIMUM_LIQUIDITY {
            return Err(AmmError::DepositTooSmall.into());
        }
        liquidity_amount -= MINIMUM_LIQUIDITY;
    }

    // Transfer tokens to the pool
    invoke(
        &transfer(
            token_program.key,
            depositor_account_a.key,
            pool_account_a.key,
            depositor.key,
            &[],
            amount_a,
        )?,
        &[
            depositor_account_a.clone(),
            pool_account_a.clone(),
            depositor.clone(),
            token_program.clone(),
        ],
    )?;

    invoke(
        &transfer(
            token_program.key,
            depositor_account_b.key,
            pool_account_b.key,
            depositor.key,
            &[],
            amount_b,
        )?,
        &[
            depositor_account_b.clone(),
            pool_account_b.clone(),
            depositor.clone(),
            token_program.clone(),
        ],
    )?;

    // Mint the liquidity to user
    invoke_signed(
        &mint_to(
            token_program.key,
            mint_liquidity.key,
            depositor_account_liquidity.key,
            pool_authority.key,
            &[],
            liquidity_amount,
        )?,
        &[
            mint_liquidity.clone(),
            depositor_account_liquidity.clone(),
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

    Ok(())
}
