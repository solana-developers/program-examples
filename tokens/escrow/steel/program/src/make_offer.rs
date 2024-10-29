use api::prelude::*;
use steel::*;


fn send_offered_tokens_to_vault(accounts: &[AccountInfo], token_a_offered_amount: u64) -> ProgramResult {


  Ok(())
}
pub fn process_make_offer(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {

    let args = MakeOffer::try_from_bytes(data)?;

    let [
    maker,
    token_mint_a,
    token_mint_b,
    maker_token_account_a,
    vault,
    offer,
    associated_token_program,
    token_program,
    system_program] =
        accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    maker.is_signer()?;

    token_mint_a
        .has_owner(&spl_token::ID)?;

    token_mint_b
        .has_owner(&spl_token::ID)?;

    maker_token_account_a
        .has_owner(&spl_token::ID)?
        .to_token_account()?
        .check(|account: &spl_token::state::Account| account.mint == *token_mint_a.key && account.owner == *maker.key)?;


    // get seeds
    let offer_seeds = &[OFFER, maker.key.as_ref(), &args.id.to_le_bytes()];

    //derive pda and check equality
    offer
        .is_empty()?
        .has_seeds(offer_seeds, args.bump, &api::ID)?;

    vault
        .has_owner(&spl_token::ID)?
        .to_token_account()?
        .check(|account| account.mint == *token_mint_a.key)?
        .check(|account| account.owner == *offer.key)?;

    // Program checks
    token_program.is_program(&spl_token::ID)?;
    system_program.is_program(&system_program::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    // cleaner way to derive offer_pda from the seeds using helper function defined in api/src/state/mod.rs
    let offer_pda = get_offer_address_from_seeds(offer_seeds);

    /*
     create account steel framework style

     Anchor does this for us when we add the init argument e.g

     #[account(
        init, <- Anchor Initializes this account
        payer = maker,
        space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>

    Below is how steel does it
    */
    create_account::<Offer>(
        offer,
        &api::ID,
        offer_seeds,
        system_program,
        maker,
    )?;

    // Todo Create vault if user has created, but they most like have to remove or check in test
    create_associated_token_account(
        maker,
        offer,
        vault,
        token_mint_a,
        system_program,
        token_program,
        associated_token_program,
    )?;


    // get vault address gotten from api/sdk.rs
    //Todo
    // let vault = get_vault_address(offer_pda, token_mint_a.key);




    // Simple SPL transfer abstracted just like the anchor example. Details at ./shared.rs
    transfer(
        maker,
        maker_token_account_a,
        vault,
        token_program,
        args.token_a_offered_amount,
    )?;

    // Save offer details
    let offer_data = Offer {
        id: args.id,
        maker: *maker.key,
        token_mint_a: *token_mint_a.key,
        token_mint_b: *token_mint_b.key,
        token_b_wanted_amount: args.token_b_wanted_amount,
        bump: args.bump,
    };

    /*
      Potential point of confusion for rust noobs: This call below attempts to interpret the offer account's data
      -as a mutable reference to an Offer struct.
      It checks if the account is owned by the correct which is our program (api::ID).
      It verifies the account data size matches the Offer struct size.
      If successful, it returns a mutable reference to the account data as an Offer struct.

      The as_account_mut method ensures you're writing to the correct account type and owned by the correct program.

      .clone_from(&offer_data)
      This method copies all the data from offer_data into the account's data.
      It's more efficient than a simple assignment (=) as it avoids creating a temporary copy

      Without this step, any changes you make to offer_data would only exist in memory and wouldn't be persisted on-chain.
    */

    offer.to_account_mut::<Offer>(&api::ID)?.clone_from(&offer_data);

    Ok(())
}
