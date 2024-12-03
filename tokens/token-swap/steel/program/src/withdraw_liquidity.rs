use spl_math::uint::U256;
use steel::*;
use token_swap_api::prelude::*;
pub fn process_withdraw_liquidity(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [payer_info, depositor_info, pool_info, pool_authority_info, mint_liquidity_info, mint_a_info, mint_b_info, pool_account_a_info, pool_account_b_info, depositor_account_liquidity_info, depositor_account_a_info, depositor_account_b_info, token_program, system_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let args = WithdrawLiquidity::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Check payer account is signer and program is the correct program.
    payer_info.is_signer()?;
    token_program.is_program(&spl_token::ID)?;
    system_program.is_program(&system_program::ID)?;
    associated_token_program.is_program(&ASSOCIATED_TOKEN_PROGRAM_ID)?;

    // check if depositor is signer
    depositor_info.is_signer()?;

    // Verify mint_a and mint_b is a mint account.
    let _mint_a = mint_a_info.as_mint()?;
    let _mint_b = mint_b_info.as_mint()?;

    // validate pool account

    if pool_info.data_is_empty() {
        return Err(TokenSwapError::AccountIsNotExisted.into());
    }
    validate_pool_account(pool_info, *mint_a_info.key, *mint_b_info.key)?;

    let pool_info_data = pool_info.as_account_mut::<Pool>(&token_swap_api::ID)?;

    // validate pool authority
    validate_pool_authority(
        pool_info_data,
        pool_authority_info,
        *mint_a_info.key,
        *mint_b_info.key,
    )?;

    // validate mint liquidity
    validate_mint_liquidity(
        pool_info_data,
        mint_liquidity_info,
        *mint_a_info.key,
        *mint_b_info.key,
    )?;

    // // validate pool_account_a_info, pool_account_b_info
    let pool_account_a = pool_account_a_info
        .is_writable()?
        .as_associated_token_account(pool_authority_info.key, mint_a_info.key)?;
    let pool_account_b = pool_account_b_info
        .is_writable()?
        .as_associated_token_account(pool_authority_info.key, mint_b_info.key)?;

    // validate depositor_account_a_info and depositor_account_b_info
    let _depositor_account_a = depositor_account_a_info
        .is_writable()?
        .as_associated_token_account(depositor_info.key, mint_a_info.key)?;
    let _depositor_account_b = depositor_account_b_info
        .is_writable()?
        .as_associated_token_account(depositor_info.key, mint_b_info.key)?;

    let pool_authority_seeds = &[
        pool_info_data.amm.as_ref(),
        pool_info_data.mint_a.as_ref(),
        pool_info_data.mint_b.as_ref(),
        AUTHORITY_SEED,
    ];

    // Transfer tokens from the pool
    let mint_liquidity = mint_liquidity_info.as_mint()?;
    let amount_a = U256::from(amount)
        .checked_mul(U256::from(pool_account_a.amount))
        .unwrap()
        .checked_div(U256::from(mint_liquidity.supply + MINIMUM_LIQUIDITY))
        .unwrap()
        .as_u64();

    transfer_signed_with_bump(
        pool_authority_info,
        pool_account_a_info,
        depositor_account_a_info,
        token_program,
        amount_a,
        pool_authority_seeds,
        pool_info_data.pool_authority_bump,
    )?;

    let amount_b = U256::from(amount)
        .checked_mul(U256::from(pool_account_b.amount))
        .unwrap()
        .checked_div(U256::from(mint_liquidity.supply + MINIMUM_LIQUIDITY))
        .unwrap()
        .as_u64();

    transfer_signed_with_bump(
        pool_authority_info,
        pool_account_b_info,
        depositor_account_b_info,
        token_program,
        amount_b,
        pool_authority_seeds,
        pool_info_data.pool_authority_bump,
    )?;

    // Burn the liquidity tokens
    // It will fail if the amount is invalid
    burn(
        depositor_account_liquidity_info,
        mint_liquidity_info,
        depositor_info,
        token_program,
        amount,
    )?;

    Ok(())
}
