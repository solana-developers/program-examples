use fixed::types::I64F64;
use solana_program::msg;
use steel::*;
use token_swap_api::prelude::*;

pub fn process_swap(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = SwapExactTokens::try_from_bytes(data)?;
    let swap_a = args.swap_a == 1;
    let input_amount = u64::from_le_bytes(args.input_amount);
    let min_output_amount = u64::from_le_bytes(args.min_output_amount);

    // Load accounts.
    let [signer_info, trader_info, amm_info, pool_info, pool_authority_info, mint_a_info, mint_b_info, pool_account_a, pool_account_b, trader_account_a, trader_account_b, token_program, associated_token_program, system_program] =
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
    trader_info.is_signer()?;
    amm_info.has_seeds(&get_seeds(false, false, true), &token_swap_api::ID)?;
    pool_info.as_account::<Pool>(&token_swap_api::ID)?;
    pool_info.has_seeds(&get_seeds(false, false, false), &token_swap_api::ID)?;
    pool_authority_info.has_seeds(&get_seeds(false, true, false), &token_swap_api::ID)?;
    assert(
        !pool_authority_info.is_writable,
        TutorialError::ValidationBreached,
        "Pool authority account should be read-only",
    )?;
    mint_a_info.as_mint()?;
    mint_b_info.as_mint()?;
    pool_account_a.as_associated_token_account(pool_authority_info.key, mint_a_info.key)?;
    pool_account_b.as_associated_token_account(pool_authority_info.key, mint_b_info.key)?;

    //Initialize trader_info associated token account if needed
    let trader_token_account_a_amount =
        match trader_account_a.as_associated_token_account(trader_info.key, mint_a_info.key) {
            Ok(_) => {
                trader_account_a
                    .as_associated_token_account(trader_info.key, mint_a_info.key)?
                    .amount
            }
            Err(_) => {
                create_associated_token_account(
                    signer_info,
                    trader_info,
                    trader_account_a,
                    mint_a_info,
                    system_program,
                    token_program,
                    associated_token_program,
                )?;
                trader_account_b
                    .as_associated_token_account(trader_info.key, mint_b_info.key)?
                    .amount
            }
        };
    let trader_token_account_b_amount =
        match trader_account_b.as_associated_token_account(trader_info.key, mint_b_info.key) {
            Ok(_) => {
                trader_account_b
                    .as_associated_token_account(trader_info.key, mint_b_info.key)?
                    .amount
            }
            Err(_) => {
                create_associated_token_account(
                    signer_info,
                    trader_info,
                    trader_account_b,
                    mint_b_info,
                    system_program,
                    token_program,
                    associated_token_program,
                )?;
                trader_account_b
                    .as_associated_token_account(trader_info.key, mint_b_info.key)?
                    .amount
            }
        };
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Prevent depositing assets the depositor does not own
    let input = if swap_a && input_amount > trader_token_account_a_amount {
        trader_token_account_a_amount
    } else if !swap_a && input_amount > trader_token_account_b_amount {
        trader_token_account_b_amount
    } else {
        input_amount
    };

    // Apply trading fee, used to compute the output
    let amm_info = amm_info.as_account::<Amm>(&token_swap_api::ID)?;
    let taxed_input = input - input * amm_info.fee as u64 / 10000;

    let pool_a =
        &pool_account_a.as_associated_token_account(pool_authority_info.key, mint_a_info.key)?;
    let pool_b =
        &pool_account_b.as_associated_token_account(pool_authority_info.key, mint_b_info.key)?;

    let output = if swap_a {
        I64F64::from_num(taxed_input)
            .checked_mul(I64F64::from_num(pool_b.amount))
            .unwrap()
            .checked_div(
                I64F64::from_num(pool_a.amount)
                    .checked_add(I64F64::from_num(taxed_input))
                    .unwrap(),
            )
            .unwrap()
    } else {
        I64F64::from_num(taxed_input)
            .checked_mul(I64F64::from_num(pool_a.amount))
            .unwrap()
            .checked_div(
                I64F64::from_num(pool_b.amount)
                    .checked_add(I64F64::from_num(taxed_input))
                    .unwrap(),
            )
            .unwrap()
    }
    .to_num::<u64>();

    if output < min_output_amount {
        TutorialError::OutputTooSmall.print();
        return Err(TutorialError::OutputTooSmall.into());
    };

    // Compute the invariant before the trade
    let invariant = pool_a.amount * pool_b.amount;

    // - Program Logic - Swapping token A for B

    // Transfer tokens to the pool_info
    let bump = Pubkey::find_program_address(&get_seeds(false, true, false), &token_swap_api::ID).1;

    if swap_a {
        transfer(
            trader_info,
            trader_account_a,
            pool_account_a,
            token_program,
            input,
        )?;
        transfer_signed_with_bump(
            pool_authority_info,
            pool_account_b,
            trader_account_b,
            token_program,
            output,
            &get_seeds(false, true, false),
            bump,
        )?
    } else {
        transfer(
            trader_info,
            trader_account_b,
            pool_account_b,
            token_program,
            input,
        )?;
        transfer_signed_with_bump(
            pool_authority_info,
            pool_account_a,
            trader_account_a,
            token_program,
            output,
            &get_seeds(false, true, false),
            bump,
        )?
    }

    msg!(
        "Traded {} tokens ({} after fees) for {}",
        input,
        taxed_input,
        output
    );

    // Reload accounts because of the CPIs
    let pool_a =
        &pool_account_a.as_associated_token_account(pool_authority_info.key, mint_a_info.key)?;
    let pool_b =
        &pool_account_b.as_associated_token_account(pool_authority_info.key, mint_b_info.key)?;

    // Verify the invariant still holds
    // We tolerate if the new invariant is higher because it means a rounding error for LPs
    assert(
        invariant < (pool_a.amount * pool_b.amount),
        TutorialError::InvariantViolated,
        &TutorialError::InvariantViolated.to_string(),
    )?;
    Ok(())
}
