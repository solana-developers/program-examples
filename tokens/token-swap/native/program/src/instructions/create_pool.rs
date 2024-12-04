use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::{self},
};

use spl_token::{solana_program::program_pack::Pack, state::Mint};

use crate::{errors::AmmError, state::Pool};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CreatePoolArgs {}

pub fn process_create_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _args: CreatePoolArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let amm = next_account_info(accounts_iter)?;
    let pool = next_account_info(accounts_iter)?;
    let pool_authority = next_account_info(accounts_iter)?;
    let mint_liquidity = next_account_info(accounts_iter)?;
    let mint_a = next_account_info(accounts_iter)?;
    let mint_b = next_account_info(accounts_iter)?;
    let pool_account_a = next_account_info(accounts_iter)?;
    let pool_account_b = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;

    // Enforce mint key ordering to prevent duplicate pools
    if mint_a.key >= mint_b.key {
        return Err(AmmError::InvalidMint.into());
    }

    // Create pool account
    let pool_bump = Pubkey::find_program_address(
        &[
            Pool::SEED_PREFIX.as_bytes(),
            amm.key.as_ref(),
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
        ],
        program_id,
    )
    .1;

    invoke_signed(
        &system_instruction::create_account(
            &payer.key,
            &pool.key,
            Rent::default().minimum_balance(Pool::space()),
            Pool::space() as u64,
            program_id,
        ),
        &[payer.clone(), pool.clone(), system_program.clone()],
        &[&[
            Pool::SEED_PREFIX.as_bytes(),
            amm.key.as_ref(),
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
            &[pool_bump],
        ]],
    )?;

    let pool_data = Pool {
        amm: *amm.key,
        mint_a: *mint_a.key,
        mint_b: *mint_b.key,
    };

    pool_data.serialize(&mut &mut pool.data.borrow_mut()[..])?;

    // Create mint_liquidity account (mint for liquidity tokens)
    // and initialize mint
    let mint_liquidity_bump = Pubkey::find_program_address(
        &[
            Pool::LIQUIDITY_PREFIX.as_bytes(),
            amm.key.as_ref(),
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
        ],
        program_id,
    )
    .1;

    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            mint_liquidity.key,
            Rent::default().minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            token_program.key,
        ),
        &[
            payer.clone(),
            mint_liquidity.clone(),
            system_program.clone(),
        ],
        &[&[
            Pool::LIQUIDITY_PREFIX.as_bytes(),
            amm.key.as_ref(),
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
            &[mint_liquidity_bump],
        ]],
    )?;

    invoke(
        &spl_token::instruction::initialize_mint(
            token_program.key,
            mint_liquidity.key,
            pool_authority.key,
            Some(pool_authority.key),
            6,
        )?,
        &[mint_liquidity.clone(), rent.clone(), token_program.clone()],
    )?;

    // Create associated token accounts for the tokens in the pool
    invoke(
        &spl_associated_token_account::instruction::create_associated_token_account(
            payer.key,
            pool_authority.key,
            mint_a.key,
            token_program.key,
        ),
        &[
            payer.clone(),
            pool_account_a.clone(),
            pool_authority.clone(),
            mint_a.clone(),
            system_program.clone(),
            token_program.clone(),
            associated_token_program.clone(),
        ],
    )?;

    invoke(
        &spl_associated_token_account::instruction::create_associated_token_account(
            payer.key,
            pool_authority.key,
            mint_b.key,
            token_program.key,
        ),
        &[
            payer.clone(),
            pool_account_b.clone(),
            pool_authority.clone(),
            mint_b.clone(),
            system_program.clone(),
            token_program.clone(),
            associated_token_program.clone(),
        ],
    )?;

    Ok(())
}
