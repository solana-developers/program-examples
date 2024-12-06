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
use spl_token::{instruction::transfer, state::Account as TokenAccount};

use crate::{
    errors::AmmError,
    state::{Amm, Pool},
};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct SwapExactTokensForTokensArgs {
    swap_a: bool,
    input_amount: u64,
    min_output_amount: u64,
}

pub fn process_swap_exact_tokens_for_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: SwapExactTokensForTokensArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let amm = next_account_info(accounts_iter)?;
    let pool = next_account_info(accounts_iter)?;
    let pool_authority = next_account_info(accounts_iter)?;
    let trader = next_account_info(accounts_iter)?;
    let mint_a = next_account_info(accounts_iter)?;
    let mint_b = next_account_info(accounts_iter)?;
    let pool_account_a = next_account_info(accounts_iter)?;
    let pool_account_b = next_account_info(accounts_iter)?;
    let trader_account_a = next_account_info(accounts_iter)?;
    let trader_account_b = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // Check that the pool corresponds to the amm and target mints
    let pool_data = Pool::try_from_slice(&pool.data.borrow())?;
    if &pool_data.amm != amm.key
        || &pool_data.mint_a != mint_a.key
        || &pool_data.mint_b != mint_b.key
    {
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
    let trader_token_account_data_a = TokenAccount::unpack(&trader_account_a.data.borrow())?;
    let trader_token_account_data_b = TokenAccount::unpack(&trader_account_b.data.borrow())?;
    let input = if args.swap_a && args.input_amount > trader_token_account_data_a.amount {
        trader_token_account_data_a.amount
    } else if !args.swap_a && args.input_amount > trader_token_account_data_b.amount {
        trader_token_account_data_b.amount
    } else {
        args.input_amount
    };

    // Apply trading fee, used to compute the output
    let amm_data = Amm::try_from_slice(&amm.data.borrow())?;
    let taxed_input = input - input * amm_data.fee as u64 / 10000;

    let pool_token_account_data_a = TokenAccount::unpack(&pool_account_a.data.borrow())?;
    let pool_token_account_data_b = TokenAccount::unpack(&pool_account_b.data.borrow())?;
    let output = if args.swap_a {
        I64F64::from_num(taxed_input)
            .checked_mul(I64F64::from_num(pool_token_account_data_b.amount))
            .unwrap()
            .checked_div(
                I64F64::from_num(pool_token_account_data_a.amount)
                    .checked_add(I64F64::from_num(taxed_input))
                    .unwrap(),
            )
            .unwrap()
    } else {
        I64F64::from_num(taxed_input)
            .checked_mul(I64F64::from_num(pool_token_account_data_a.amount))
            .unwrap()
            .checked_div(
                I64F64::from_num(pool_token_account_data_b.amount)
                    .checked_add(I64F64::from_num(taxed_input))
                    .unwrap(),
            )
            .unwrap()
    }
    .to_num::<u64>();

    if output < args.min_output_amount {
        return Err(AmmError::OutputTooSmall.into());
    }

    // Compute the invariant before the trade
    let invariant = pool_token_account_data_a.amount * pool_token_account_data_b.amount;

    // Transfer tokens to the pool
    if args.swap_a {
        invoke(
            &transfer(
                token_program.key,
                trader_account_a.key,
                pool_account_a.key,
                trader.key,
                &[],
                input,
            )?,
            &[
                trader_account_a.clone(),
                pool_account_a.clone(),
                trader.clone(),
                token_program.clone(),
            ],
        )?;
        invoke_signed(
            &transfer(
                token_program.key,
                pool_account_b.key,
                trader_account_b.key,
                pool_authority.key,
                &[],
                output,
            )?,
            &[
                trader_account_b.clone(),
                pool_account_b.clone(),
                pool_authority.clone(),
                token_program.clone(),
            ],
            &[&[
                Pool::AUTHORITY_PREFIX.as_ref(),
                amm.key.as_ref(),
                mint_a.key.as_ref(),
                mint_b.key.as_ref(),
                &[pool_authority_bump],
            ]],
        )?;
    } else {
        invoke(
            &transfer(
                token_program.key,
                trader_account_b.key,
                pool_account_b.key,
                trader.key,
                &[],
                input,
            )?,
            &[
                trader_account_b.clone(),
                pool_account_b.clone(),
                trader.clone(),
                token_program.clone(),
            ],
        )?;
        invoke_signed(
            &transfer(
                token_program.key,
                pool_account_a.key,
                trader_account_a.key,
                pool_authority.key,
                &[],
                output,
            )?,
            &[
                trader_account_a.clone(),
                pool_account_a.clone(),
                pool_authority.clone(),
                token_program.clone(),
            ],
            &[&[
                Pool::AUTHORITY_PREFIX.as_ref(),
                amm.key.as_ref(),
                mint_a.key.as_ref(),
                mint_b.key.as_ref(),
                &[pool_authority_bump],
            ]],
        )?;
    }

    // Verify the invariant still holds
    // Reload accounts because of the CPIs
    // We tolerate if the new invariant is higher because it means a rounding error for LPs
    let pool_token_account_data_a = TokenAccount::unpack(&pool_account_a.data.borrow())?;
    let pool_token_account_data_b = TokenAccount::unpack(&pool_account_b.data.borrow())?;
    if invariant > pool_token_account_data_a.amount * pool_token_account_data_b.amount {
        return Err(AmmError::InvariantViolated.into());
    }

    Ok(())
}
