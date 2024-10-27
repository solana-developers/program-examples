use escrow_api::prelude::*;
use steel::*;

pub fn process_take_offer(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [taker_signer, maker, token_mint_a, token_mint_b, taker_token_account_a, taker_token_account_b, maker_token_account_b, offer, vault, token_program, associated_token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    taker_signer.is_signer()?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    // create ATA's for maker_token_b and taker_token_a
    create_associated_token_account(
        taker_signer,
        maker,
        maker_token_account_b,
        token_mint_b,
        system_program,
        token_program,
        associated_token_program,
    )?;
    create_associated_token_account(
        taker_signer,
        taker_signer,
        taker_token_account_a,
        token_mint_a,
        system_program,
        token_program,
        associated_token_program,
    )?;

    let offer_data: &Offer = offer.as_account::<Offer>(&escrow_api::ID)?;

    // Move the tokens from the taker's ATA to the maker's ATA
    transfer(
        taker_signer,
        taker_token_account_b,
        maker_token_account_b,
        token_program,
        offer_data.token_b_wanted_amount,
    )?;

    let id = offer_data.id.to_le_bytes();

    let seeds = &[OFFER, maker.key.as_ref(), &id];
    let signer_seeds = &seeds[..];

    // transfer from vault to taker
    transfer_signed(
        offer,
        vault,
        taker_token_account_a,
        token_program,
        vault
            .as_associated_token_account(offer.key, token_mint_a.key)?
            .amount,
        signer_seeds,
    )?;
    let close_instruction = &spl_token::instruction::close_account(
        token_program.key,
        vault.key,
        taker_signer.key,
        offer.key,
        &[offer.key, taker_signer.key],
    )?;
    let account_infos = &[vault.clone(), taker_signer.clone(), offer.clone()];

    invoke_signed_with_bump(
        close_instruction,
        account_infos,
        signer_seeds,
        offer_data.bump as u8,
    )?;

    Ok(())
}
