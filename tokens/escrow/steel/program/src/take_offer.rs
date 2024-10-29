use api::prelude::*;
use steel::*;


pub fn process_take_offer(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {

    let args = TakeOffer::try_from_bytes(data)?;
    let [
    taker,
    maker, token_mint_a, token_mint_b, taker_token_account_a, taker_token_account_b, maker_token_account_b, offer, vault, associated_token_program, token_program, system_program] =
        accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    taker.is_signer()?;
    // let offer_data = offer.as_account::<Offer>(program_id)?;
    //
    // // Transfer wanted tokens to maker
    // transfer(
    //     taker,
    //     taker_token_account_b,
    //     maker_token_account_b,
    //     token_program,
    //     offer_data.token_b_wanted_amount,
    // )?;
    //
    // // Transfer offered tokens to taker
    // let offer_seeds = &[
    //     b"offer",
    //     maker.key.as_ref(),
    //     &offer_data.id.to_le_bytes(),
    //     &[offer_data.bump],
    // ];
    // transfer_signed(
    //     offer,
    //     vault,
    //     taker_token_account_a,
    //     token_program,
    //     vault.as_account::<spl_token::state::Account>(program_id)?.amount,
    //     offer_seeds,
    // )?;
    //
    // // Close vault
    // close_account(vault, maker)?;
    //
    // // Close offer account
    // offer.close(maker)?;
    //
    Ok(())
}
