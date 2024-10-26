use solana_program::program_pack::Pack;
use spl_token::state::Mint;
use steel::*;
use token_swap_api::prelude::*;

pub fn process_create_pool(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [payer_info, amm_info, pool_info, pool_authority_info, mint_liquidity_info, mint_a_info, mint_b_info, pool_account_a_info, pool_account_b_info, token_program, system_program, rent_sysvar] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Check payer account is signer.
    payer_info.is_signer()?;
    token_program.is_program(&spl_token::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Check amm account is owned by token_swap_api::ID.
    amm_info.has_owner(&token_swap_api::ID)?;

    // Check pool account is owned by token_swap_api::ID.
    pool_info.is_empty()?.is_writable()?.has_seeds(
        &[
            &amm_info.key.to_bytes(),
            &mint_a_info.key.to_bytes(),
            &mint_b_info.key.to_bytes(),
        ],
        &token_swap_api::ID,
    )?;

    // Check pool_authority account
    pool_authority_info.is_empty()?.has_seeds(
        &[
            &amm_info.key.to_bytes(),
            &mint_a_info.key.to_bytes(),
            &mint_b_info.key.to_bytes(),
            &AUTHORITY_SEED,
        ],
        &token_swap_api::ID,
    )?;

    // Check mint_liquidity account
    mint_liquidity_info.is_empty()?.is_writable()?.has_seeds(
        &[
            &amm_info.key.to_bytes(),
            &mint_a_info.key.to_bytes(),
            &mint_b_info.key.to_bytes(),
            &LIQUIDITY_SEED,
        ],
        &token_swap_api::ID,
    )?;

    // Verify mint_a and mint_b is a mint account.
    let _mint_a = mint_a_info.as_mint()?;
    let _mint_b = mint_b_info.as_mint()?;

    // Verify pool_account_a and pool_account_b is a token account.
    pool_account_a_info.is_empty()?.is_writable()?;
    // .as_associated_token_account(&spl_token::ID, mint_a_info.key)?;
    pool_account_b_info.is_empty()?.is_writable()?;
    // .as_associated_token_account(pool_authority_info.key, mint_b_info.key)?;

    // init pool account
    create_account::<Pool>(
        pool_info,
        system_program,
        payer_info,
        &token_swap_api::ID,
        &[
            &amm_info.key.to_bytes(),
            &mint_a_info.key.to_bytes(),
            &mint_b_info.key.to_bytes(),
        ],
    )?;

    let pool_info_data = pool_info.as_account_mut::<Pool>(&token_swap_api::ID)?;
    pool_info_data.amm = *amm_info.key;
    pool_info_data.mint_a = *mint_a_info.key;
    pool_info_data.mint_b = *mint_b_info.key;

    let (_, bump) = mint_liquidity_pda(*amm_info.key, *mint_a_info.key, *mint_b_info.key);
    // allocate mint_liquidity account
    allocate_account_with_bump(
        mint_liquidity_info,
        system_program,
        payer_info,
        Mint::LEN,
        &spl_token::ID,
        &[
            amm_info.key.as_ref(),
            mint_a_info.key.as_ref(),
            mint_b_info.key.as_ref(),
            LIQUIDITY_SEED,
        ],
        bump,
    )?;

    // init mint_liquidity account
    solana_program::program::invoke(
        &spl_token::instruction::initialize_mint(
            &spl_token::ID,
            mint_liquidity_info.key,
            pool_authority_info.key,
            Some(pool_authority_info.key),
            9,
        )?,
        &[
            token_program.clone(),
            mint_liquidity_info.clone(),
            pool_authority_info.clone(),
            rent_sysvar.clone(),
        ],
    )?;
    Ok(())
}
