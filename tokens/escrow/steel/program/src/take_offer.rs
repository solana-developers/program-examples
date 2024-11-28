use api::prelude::*;
use steel::*;

pub fn process_take_offer(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let args = TakeOffer::try_from_bytes(data)?;
    let [
    taker,
    maker,
    token_mint_a,
    token_mint_b,
    taker_token_account_a,
    taker_token_account_b,
    maker_token_account_b,
    offer,
    vault,
    associated_token_program,
    token_program,
    system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    taker.is_signer()?;
    
    token_mint_a.has_owner(&spl_token::ID)?;
    token_mint_b.has_owner(&spl_token::ID)?;

    // Validate token accounts
    taker_token_account_a
        .has_owner(&spl_token::ID)?
        .to_token_account()?
        .check(|account| account.mint == *token_mint_a.key && account.owner == *taker.key)?;

    taker_token_account_b
        .has_owner(&spl_token::ID)?
        .to_token_account()?
        .check(|account| account.mint == *token_mint_b.key && account.owner == *taker.key)?;

    maker_token_account_b
        .has_owner(&spl_token::ID)?
        .to_token_account()?
        .check(|account| account.mint == *token_mint_b.key && account.owner == *maker.key)?;

    // Get the offer data
    let offer_data = offer.as_account::<Offer>(&api::ID)?;

    // Verify offer details
    assert_eq!(offer_data.maker, *maker.key);
    assert_eq!(offer_data.token_mint_a, *token_mint_a.key);
    assert_eq!(offer_data.token_mint_b, *token_mint_b.key);

    // Verify vault
    vault
        .has_owner(&spl_token::ID)?
        .to_token_account()?
        .check(|account| account.mint == *token_mint_a.key && account.owner == *offer.key)?;

    // Program checks
    token_program.is_program(&spl_token::ID)?;
    system_program.is_program(&system_program::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    // Transfer wanted tokens from taker to maker
    transfer(
        taker,
        taker_token_account_b,
        maker_token_account_b,
        token_program,
        offer_data.token_b_wanted_amount,
    )?;

    // Transfer offered tokens to taker
    let offer_seeds = &[
        OFFER,
        maker.key.as_ref(),
        &offer_data.id.to_le_bytes(),
        &[offer_data.bump],
    ];

    transfer_signed(
        offer,
        vault,
        taker_token_account_a,
        token_program,
        vault.to_token_account()?.amount,
        offer_seeds,
    )?;

    // Close vault account
    close_token_account(
        vault,
        maker,
        offer,
        token_program,
        offer_seeds,
    )?;

    // Close offer account and return rent
    offer.close(maker)?;

    Ok(())
}

// Complete the send_offered_tokens_to_vault function
fn send_offered_tokens_to_vault(accounts: &[AccountInfo], token_a_offered_amount: u64) -> ProgramResult {
    let [maker, maker_token_account_a, vault, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    maker.is_signer()?;
    maker_token_account_a.has_owner(&spl_token::ID)?;
    vault.has_owner(&spl_token::ID)?;
    token_program.is_program(&spl_token::ID)?;

    // Transfer tokens to vault
    transfer(
        maker,
        maker_token_account_a,
        vault,
        token_program,
        token_a_offered_amount,
    )?;

    Ok(())
}
