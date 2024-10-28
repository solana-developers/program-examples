// use solana_program::msg;
use solana_program::program_pack::Pack;
use spl_token::state::Mint;
use steel::*;
use token_swap_api::prelude::*;

pub fn process_create_pool(accounts: &[AccountInfo<'_>]) -> ProgramResult {
    // Load accounts.
    let [signer, amm, pool, pool_authority, mint_liquidity, mint_a, mint_b, pool_account_a, pool_account_b, token_program, associated_token_program, rent_account, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let get_seeds = |include_liquidity: bool, include_authority: bool| -> Vec<&[u8]> {
        let mut seeds = vec![amm.key.as_ref(), mint_a.key.as_ref(), mint_b.key.as_ref()];

        if include_liquidity {
            seeds.push(LIQUIDITY_SEED);
        }

        if include_authority {
            seeds.push(AUTHORITY_SEED);
        }

        seeds
    };

    create_account::<Pool>(
        pool,
        system_program,
        signer,
        &token_swap_api::ID,
        &get_seeds(false, false),
    )?;

    let bump = Pubkey::find_program_address(&get_seeds(true, false), &token_swap_api::ID).1;
    allocate_account_with_bump(
        mint_liquidity,
        system_program,
        signer,
        Mint::LEN,
        &token_program.key,
        &get_seeds(true, false),
        bump,
    )?;
    initialize_mint(
        mint_liquidity,
        pool_authority,
        None,
        token_program,
        rent_account,
        6,
    )?;

    create_associated_token_account(
        signer,
        pool_authority,
        pool_account_a,
        mint_a,
        system_program,
        token_program,
        associated_token_program,
    )?;
    create_associated_token_account(
        signer,
        pool_authority,
        pool_account_b,
        mint_b,
        system_program,
        token_program,
        associated_token_program,
    )?;
    let pool_data: &mut Pool = pool.as_account_mut::<Pool>(&token_swap_api::ID)?;

    pool_data.amm = *amm.key;
    pool_data.mint_a = *mint_a.key;
    pool_data.mint_b = *mint_b.key;

    Ok(())
}
