// use solana_program::msg;
use solana_program::program_pack::Pack;
use spl_token::state::Mint;
use steel::*;
use token_swap_api::prelude::*;

pub fn process_create_pool(accounts: &[AccountInfo<'_>]) -> ProgramResult {
    // Load accounts.
    let [signer_info, amm_info, pool_info, pool_authority_info, mint_liquidity_info, mint_a_info, mint_b_info, pool_account_a, pool_account_b, token_program, associated_token_program, rent_account, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    amm_info.has_owner(&token_swap_api::ID)?;
    // helper closure to get seeds for different accounts
    let get_seeds = |include_liquidity: bool, include_authority: bool| -> Vec<&[u8]> {
        let mut seeds = vec![
            amm_info.key.as_ref(),
            mint_a_info.key.as_ref(),
            mint_b_info.key.as_ref(),
        ];

        if include_liquidity {
            seeds.push(LIQUIDITY_SEED);
        }

        if include_authority {
            seeds.push(AUTHORITY_SEED);
        }

        seeds
    };

    //extracting amm_account_data
    let amm_data = amm_info.as_account_mut::<Amm>(&token_swap_api::ID)?;

    // validating accounts
    signer_info.is_signer()?;
    amm_info.has_seeds(&[amm_data.id.as_ref()], &token_swap_api::ID)?;
    pool_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&get_seeds(false, false), &token_swap_api::ID)?;
    pool_authority_info.has_seeds(&get_seeds(false, true), &token_swap_api::ID)?;
    assert(
        !pool_authority_info.is_writable,
        TutorialError::ValidationBreached,
        "Pool authority account should be read-only",
    )?;
    mint_liquidity_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&get_seeds(true, false), &token_swap_api::ID)?;
    mint_a_info.as_mint()?;
    mint_b_info.as_mint()?;
    pool_account_a.is_empty()?.is_writable()?;
    pool_account_b.is_empty()?.is_writable()?;

    //Validating programs
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    // - Program Logic - create accounts and populate fields

    create_account::<Pool>(
        pool_info,
        system_program,
        signer_info,
        &token_swap_api::ID,
        &get_seeds(false, false),
    )?;

    let bump = Pubkey::find_program_address(&get_seeds(true, false), &token_swap_api::ID).1;
    allocate_account_with_bump(
        mint_liquidity_info,
        system_program,
        signer_info,
        Mint::LEN,
        &token_program.key,
        &get_seeds(true, false),
        bump,
    )?;
    initialize_mint(
        mint_liquidity_info,
        pool_authority_info,
        None,
        token_program,
        rent_account,
        6,
    )?;

    create_associated_token_account(
        signer_info,
        pool_authority_info,
        pool_account_a,
        mint_a_info,
        system_program,
        token_program,
        associated_token_program,
    )?;
    create_associated_token_account(
        signer_info,
        pool_authority_info,
        pool_account_b,
        mint_b_info,
        system_program,
        token_program,
        associated_token_program,
    )?;
    let pool_data: &mut Pool = pool_info.as_account_mut::<Pool>(&token_swap_api::ID)?;

    pool_data.amm = *amm_info.key;
    pool_data.mint_a = *mint_a_info.key;
    pool_data.mint_b = *mint_b_info.key;

    Ok(())
}
