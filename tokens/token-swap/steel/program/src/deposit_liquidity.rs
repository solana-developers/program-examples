// use solana_program::msg;
use fixed::types::I64F64;
use solana_program::msg;
use steel::*;
use token_swap_api::prelude::*;

pub fn process_deposit(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = DepositLiquidity::try_from_bytes(data)?;
    let amount_a = u64::from_le_bytes(args.amount_a);
    let amount_b = u64::from_le_bytes(args.amount_b);

    // Load accounts.
    let [signer, depositor, pool, pool_authority, mint_liquidity, mint_a, mint_b, pool_account_a, pool_account_b, depositor_liquidity, depositor_account_a, depositor_account_b, token_program, associated_token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    //extracting pool_account_data
    let pool_data: &mut Pool = pool.as_account_mut::<Pool>(&token_swap_api::ID)?;

    // helper closure to get seeds for different accounts
    let get_seeds = |include_liquidity: bool, include_authority: bool| -> Vec<&[u8]> {
        let mut seeds = vec![
            pool_data.amm.as_ref(),
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
        ];

        if include_liquidity {
            seeds.push(LIQUIDITY_SEED);
        }

        if include_authority {
            seeds.push(AUTHORITY_SEED);
        }

        seeds
    };

    //Validate Accounts
    signer.is_signer()?;
    depositor.is_signer()?;
    pool.as_account::<Pool>(&token_swap_api::ID)?;
    pool.has_seeds(&get_seeds(false, false), &token_swap_api::ID)?;
    assert!(pool_data.mint_a == *mint_a.key);
    assert!(pool_data.mint_b == *mint_b.key);
    pool_authority.has_seeds(&get_seeds(false, true), &token_swap_api::ID)?;
    mint_liquidity.has_seeds(&get_seeds(true, false), &token_swap_api::ID)?;
    pool_account_a.as_associated_token_account(pool_authority.key, mint_a.key)?;
    pool_account_b.as_associated_token_account(pool_authority.key, mint_b.key)?;

    //create depositor_liquidity associated_token_account if needed
    match depositor_liquidity.as_associated_token_account(depositor.key, mint_liquidity.key) {
        Ok(_) => msg!("depositor_liquidity_token_account already exists"),
        Err(_) => {
            create_associated_token_account(
                signer,
                depositor,
                depositor_liquidity,
                mint_liquidity,
                system_program,
                token_program,
                associated_token_program,
            )?;
        }
    }

    //validate depositor token accounts and save amount
    let depositor_token_account_a_amount =
        depositor_account_a.as_associated_token_account(depositor.key, mint_a.key)?.amount;
    let depositor_token_account_b_amount =
        depositor_account_b.as_associated_token_account(depositor.key, mint_b.key)?.amount;

    // Prevent depositing assets the depositor does not own
    let mut amount_a = if amount_a > depositor_token_account_a_amount {
        depositor_token_account_a_amount
    } else {
        amount_a
    };
    let mut amount_b = if amount_b > depositor_token_account_b_amount {
        depositor_token_account_b_amount
    } else {
        amount_b
    };

    // Making sure they are provided in the same proportion as existing liquidity
    let pool_a = &pool_account_a.as_associated_token_account(pool_authority.key, mint_a.key)?;
    let pool_b = &pool_account_b.as_associated_token_account(pool_authority.key, mint_b.key)?;

    // Defining pool creation like this allows attackers to frontrun pool creation with bad ratios
    let pool_creation = pool_a.amount == 0 && pool_b.amount == 0;
    (amount_a, amount_b) = if pool_creation {
        // Add as is if there is no liquidity
        (amount_a, amount_b)
    } else {
        let ratio = I64F64::from_num(pool_a.amount)
            .checked_mul(I64F64::from_num(pool_b.amount))
            .unwrap();
        if pool_a.amount > pool_b.amount {
            (
                I64F64::from_num(amount_b)
                    .checked_mul(ratio)
                    .unwrap()
                    .to_num::<u64>(),
                amount_b,
            )
        } else {
            (
                amount_a,
                I64F64::from_num(amount_a)
                    .checked_div(ratio)
                    .unwrap()
                    .to_num::<u64>(),
            )
        }
    };

    // Computing the amount of liquidity about to be deposited
    let mut liquidity = I64F64::from_num(amount_a)
        .checked_mul(I64F64::from_num(amount_b))
        .unwrap()
        .sqrt()
        .to_num::<u64>();

    // Lock some minimum liquidity on the first deposit
    if pool_creation {
        if liquidity < MINIMUM_LIQUIDITY {
            return Err(TutorialError::DepositTooSmall.into());
        }

        liquidity -= MINIMUM_LIQUIDITY;
    };

    // Transfer tokens to the pool
    transfer(
        depositor,
        depositor_account_a,
        pool_account_a,
        token_program,
        amount_a,
    )?;
    transfer(
        depositor,
        depositor_account_b,
        pool_account_b,
        token_program,
        amount_b,
    )?;

    // Mint the liquidity to user
    let bump = Pubkey::find_program_address(&get_seeds(false, true), &token_swap_api::ID).1;

    mint_to_signed_with_bump(
        mint_liquidity,
        depositor_liquidity,
        pool_authority,
        token_program,
        liquidity,
        &get_seeds(false, true),
        bump,
    )?;
    Ok(())
}
