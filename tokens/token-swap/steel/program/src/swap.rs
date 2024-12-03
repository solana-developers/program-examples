use spl_math::uint::U256;
use steel::*;
use token_swap_api::prelude::*;
pub fn process_swap(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [payer_info, trader_info, amm_info, pool_info, pool_authority_info, mint_a_info, mint_b_info, pool_account_a_info, pool_account_b_info, trader_account_a_info, trader_account_b_info, token_program, system_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let args = Swap::try_from_bytes(data)?;
    let swap_a: bool = args.swap_a == 1;
    let input_amount = u64::from_le_bytes(args.input_amount);
    let min_output_amount = u64::from_le_bytes(args.min_output_amount);

    // Check payer account is signer and program is the correct program.
    payer_info.is_signer()?;
    token_program.is_program(&spl_token::ID)?;
    system_program.is_program(&system_program::ID)?;
    associated_token_program.is_program(&ASSOCIATED_TOKEN_PROGRAM_ID)?;

    // check if depositor is signer
    trader_info.is_signer()?;

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

    // Check amm account is owned by token_swap_api::ID.
    amm_info.has_owner(&token_swap_api::ID)?;

    // // validate pool_account_a_info, pool_account_b_info
    let pool_account_a = pool_account_a_info
        .is_writable()?
        .as_associated_token_account(pool_authority_info.key, mint_a_info.key)?;
    let pool_account_b = pool_account_b_info
        .is_writable()?
        .as_associated_token_account(pool_authority_info.key, mint_b_info.key)?;

    // check if trader_account_a and trader_account_b is exists

    if trader_account_a_info.data_is_empty() {
        // Create the depositor's liquidity account if it does not exist
        create_associated_token_account(
            payer_info,
            trader_info,
            trader_account_a_info,
            mint_a_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
    }

    if trader_account_b_info.data_is_empty() {
        // Create the depositor's liquidity account if it does not exist
        create_associated_token_account(
            payer_info,
            trader_info,
            trader_account_b_info,
            mint_b_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
    }

    let trader_account_a =
        trader_account_a_info.as_associated_token_account(trader_info.key, mint_a_info.key)?;
    let trader_account_b =
        trader_account_b_info.as_associated_token_account(trader_info.key, mint_b_info.key)?;

    // Prevent depositing assets the depositor does not own
    let input = if swap_a && input_amount > trader_account_a.amount {
        trader_account_a.amount
    } else if !swap_a && input_amount > trader_account_b.amount {
        trader_account_b.amount
    } else {
        input_amount
    };

    // Apply trading fee, used to compute the output
    let amm = amm_info.as_account_mut::<Amm>(&token_swap_api::ID)?;
    let fee = u16::from_le_bytes(amm.fee);
    let taxed_input = input - input * (fee as u64) / 10000;

    let output = if swap_a {
        U256::from(taxed_input)
            .checked_mul(U256::from(pool_account_b.amount))
            .unwrap()
            .checked_div(
                U256::from(pool_account_a.amount)
                    .checked_add(U256::from(taxed_input))
                    .unwrap(),
            )
            .unwrap()
    } else {
        U256::from(taxed_input)
            .checked_mul(U256::from(pool_account_a.amount))
            .unwrap()
            .checked_div(
                U256::from(pool_account_b.amount)
                    .checked_add(U256::from(taxed_input))
                    .unwrap(),
            )
            .unwrap()
    }
    .as_u64();

    if output < min_output_amount {
        return Err(TokenSwapError::OutputTooSmall.into());
    }

    let pool_authority_seeds = &[
        pool_info_data.amm.as_ref(),
        pool_info_data.mint_a.as_ref(),
        pool_info_data.mint_b.as_ref(),
        AUTHORITY_SEED,
    ];

    // // Compute the invariant before the trade
    let invariant = pool_account_a.amount * pool_account_b.amount;

    if swap_a {
        transfer(
            trader_info,
            trader_account_a_info,
            pool_account_a_info,
            token_program,
            input,
        )?;
        transfer_signed_with_bump(
            pool_authority_info,
            pool_account_b_info,
            trader_account_b_info,
            token_program,
            output,
            pool_authority_seeds,
            pool_info_data.pool_authority_bump,
        )?;
    } else {
        transfer_signed_with_bump(
            pool_authority_info,
            pool_account_a_info,
            trader_account_a_info,
            token_program,
            input,
            pool_authority_seeds,
            pool_info_data.pool_authority_bump,
        )?;
        transfer(
            trader_info,
            trader_account_b_info,
            pool_account_b_info,
            token_program,
            output,
        )?;
    }

    let pool_account_a = pool_account_a_info
        .as_associated_token_account(pool_authority_info.key, mint_a_info.key)?;
    let pool_account_b = pool_account_b_info
        .as_associated_token_account(pool_authority_info.key, mint_b_info.key)?;

    if invariant > pool_account_a.amount * pool_account_b.amount {
        return Err(TokenSwapError::InvariantViolated.into());
    }

    Ok(())
}
