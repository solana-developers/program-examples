use fixed::types::I64F64;
use solana_program::msg;
use steel::*;
use token_swap_api::prelude::*;

pub fn process_withdraw_liquidity(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = WithdrawLiquidity::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    if amount == 0 {
        msg!("Withdrawal amount in zero: leaving bank");
        return Ok(());
    }

    // Load accounts.
    let [signer_info, depositor_info, amm_info, pool_info, pool_authority_info, mint_liquidity_info, mint_a_info, mint_b_info, pool_account_a, pool_account_b, depositor_liquidity, depositor_account_a, depositor_account_b, token_program, associated_token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    //extracting account datas
    let amm_data: &mut Amm = amm_info.as_account_mut::<Amm>(&token_swap_api::ID)?;
    let pool_data: &mut Pool = pool_info.as_account_mut::<Pool>(&token_swap_api::ID)?;
    assert(
        pool_data.amm == *amm_info.key,
        TutorialError::ValidationBreached,
        "Amm is not correct",
    )?;
    assert(
        pool_data.mint_a == *mint_a_info.key,
        TutorialError::InvalidMint,
        "Mint account is invalid",
    )?;
    assert(
        pool_data.mint_b == *mint_b_info.key,
        TutorialError::InvalidMint,
        "Mint account is invalid",
    )?;

    // helper closure to get seeds for different accounts

    let get_seeds =
        |include_liquidity: bool, include_authority: bool, amm_seed: bool| -> Vec<&[u8]> {
            let mut seeds = if !amm_seed {
                vec![
                    pool_data.amm.as_ref(),
                    mint_a_info.key.as_ref(),
                    mint_b_info.key.as_ref(),
                ]
            } else {
                vec![amm_data.id.as_ref()]
            };
            if include_liquidity {
                seeds.push(LIQUIDITY_SEED);
            }

            if include_authority {
                seeds.push(AUTHORITY_SEED);
            }

            seeds
        };

    //Validating Accounts
    signer_info.is_signer()?;
    depositor_info.is_signer()?;
    amm_info.has_seeds(&get_seeds(false, false, true), &token_swap_api::ID)?;
    pool_info
        .has_seeds(&get_seeds(false, false, false), &token_swap_api::ID)?
        .as_account::<Pool>(&token_swap_api::ID)?;
    pool_authority_info.has_seeds(&get_seeds(false, true, false), &token_swap_api::ID)?;
    assert(
        !pool_authority_info.is_writable,
        TutorialError::ValidationBreached,
        "Pool authority account should be read-only",
    )?;
    let mint_liquidity_data = mint_liquidity_info
        .has_seeds(&get_seeds(true, false, false), &token_swap_api::ID)?
        .as_mint()?;
    mint_a_info.as_mint()?;
    mint_b_info.as_mint()?;
    let pool_account_a_amount = pool_account_a
        .as_associated_token_account(pool_authority_info.key, mint_a_info.key)?
        .amount;
    let pool_account_b_amount = pool_account_b
        .as_associated_token_account(pool_authority_info.key, mint_b_info.key)?
        .amount;
    depositor_account_a.as_associated_token_account(depositor_info.key, mint_a_info.key)?;
    depositor_account_b.as_associated_token_account(depositor_info.key, mint_b_info.key)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Transfer tokens from the pool_info
    let amount_a = I64F64::from_num(amount)
        .checked_mul(I64F64::from_num(pool_account_a_amount))
        .unwrap()
        .checked_div(I64F64::from_num(
            mint_liquidity_data.supply + MINIMUM_LIQUIDITY,
        ))
        .unwrap()
        .floor()
        .to_num::<u64>();

    let bump = Pubkey::find_program_address(&get_seeds(false, true, false), &token_swap_api::ID).1;

    transfer_signed_with_bump(
        pool_authority_info,
        pool_account_a,
        depositor_account_a,
        token_program,
        amount_a,
        &get_seeds(false, true, false),
        bump,
    )?;

    let amount_b = I64F64::from_num(amount)
        .checked_mul(I64F64::from_num(pool_account_b_amount))
        .unwrap()
        .checked_div(I64F64::from_num(
            mint_liquidity_data.supply + MINIMUM_LIQUIDITY,
        ))
        .unwrap()
        .floor()
        .to_num::<u64>();

    let bump = Pubkey::find_program_address(&get_seeds(false, true, false), &token_swap_api::ID).1;

    transfer_signed_with_bump(
        pool_authority_info,
        pool_account_b,
        depositor_account_b,
        token_program,
        amount_b,
        &get_seeds(false, true, false),
        bump,
    )?;

    burn(
        depositor_liquidity,
        mint_liquidity_info,
        depositor_info,
        token_program,
        amount,
    )?;

    Ok(())
}
