use spl_math::uint::U256;
use steel::*;
use token_swap_api::prelude::*;
pub fn process_deposit_liquidity(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [payer_info, depositor_info, pool_info, pool_authority_info, mint_liquidity_info, mint_a_info, mint_b_info, pool_account_a_info, pool_account_b_info, depositor_account_liquidity_info, depositor_account_a_info, depositor_account_b_info, token_program, system_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let args = DepositLiquidity::try_from_bytes(data)?;
    let amount_a = u64::from_le_bytes(args.amount_a);
    let amount_b = u64::from_le_bytes(args.amount_b);

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
    let depositor_account_a = depositor_account_a_info
        .is_writable()?
        .as_associated_token_account(depositor_info.key, mint_a_info.key)?;
    let depositor_account_b = depositor_account_b_info
        .is_writable()?
        .as_associated_token_account(depositor_info.key, mint_b_info.key)?;

    // Prevent depositing assets the depositor does not own
    let mut amount_a = if amount_a > depositor_account_a.amount {
        depositor_account_a.amount
    } else {
        amount_a
    };
    let mut amount_b = if amount_b > depositor_account_b.amount {
        depositor_account_b.amount
    } else {
        amount_b
    };

    // Defining pool creation like this allows attackers to frontrun pool creation with bad ratios
    let pool_creation = pool_account_a.amount == 0 && pool_account_b.amount == 0;

    (amount_a, amount_b) = if pool_creation {
        // Add as is if there is no liquidity
        (amount_a, amount_b)
    } else {
        let ratio = U256::from(pool_account_a.amount)
            .checked_mul(U256::from(pool_account_b.amount))
            .unwrap();
        if pool_account_a.amount > pool_account_b.amount {
            (
                U256::from(amount_b).checked_mul(ratio).unwrap().as_u64(),
                amount_b,
            )
        } else {
            (
                amount_a,
                U256::from(amount_a).checked_div(ratio).unwrap().as_u64(),
            )
        }
    };

    // Transfer tokens to the pool
    transfer(
        depositor_info,
        depositor_account_a_info,
        pool_account_a_info,
        token_program,
        amount_a,
    )?;

    transfer(
        depositor_info,
        depositor_account_b_info,
        pool_account_b_info,
        token_program,
        amount_b,
    )?;

    // Computing the amount of liquidity about to be deposited
    let mut liquidity = U256::from(amount_a)
        .checked_mul(U256::from(amount_b))
        .unwrap()
        .integer_sqrt()
        .as_u64();

    // Lock some minimum liquidity on the first deposit
    if pool_creation {
        if liquidity < MINIMUM_LIQUIDITY {
            return Err(TokenSwapError::DepositTooSmall.into());
        }

        liquidity -= MINIMUM_LIQUIDITY;
    }

    if depositor_account_liquidity_info.data_is_empty() {
        // Create the depositor's liquidity account if it does not exist
        create_associated_token_account(
            payer_info,
            depositor_info,
            depositor_account_liquidity_info,
            mint_liquidity_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
    }

    // Mint the liquidity to user
    let seeds = &[
        pool_info_data.amm.as_ref(),
        pool_info_data.mint_a.as_ref(),
        pool_info_data.mint_b.as_ref(),
        AUTHORITY_SEED,
        &[pool_info_data.pool_authority_bump],
    ];
    let signer_seeds = &[&seeds[..]];
    solana_program::program::invoke_signed(
        &spl_token::instruction::mint_to(
            &spl_token::id(),
            mint_liquidity_info.key,
            depositor_account_liquidity_info.key,
            pool_authority_info.key,
            &[pool_authority_info.key],
            liquidity,
        )?,
        &[
            token_program.clone(),
            mint_liquidity_info.clone(),
            depositor_account_liquidity_info.clone(),
            pool_authority_info.clone(),
        ],
        signer_seeds,
    )?;

    Ok(())
}
