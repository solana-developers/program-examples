use escrow_api::prelude::*;
use steel::*;

pub fn process_refund(accounts: &[AccountInfo<'_>]) -> ProgramResult {
    // Load accounts.
    let [signer_maker_info, token_mint_a, maker_token_account_a, offer, vault, token_program, associated_token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_maker_info.is_signer()?;
    let offer_data: &Offer = offer.as_account::<Offer>(&escrow_api::ID)?;
    offer.is_writable()?.has_seeds(
        &[
            OFFER,
            signer_maker_info.key.as_ref(),
            offer_data.id.to_le_bytes().as_ref(),
        ],
        &escrow_api::ID,
    )?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    let id = offer_data.id.to_le_bytes();

    let seeds = &[OFFER, signer_maker_info.key.as_ref(), &id];
    let signer_seeds = &seeds[..];

    // refund / transfer from vault to maker
    transfer_signed(
        offer,
        vault,
        maker_token_account_a,
        token_program,
        vault
            .as_associated_token_account(offer.key, token_mint_a.key)?
            .amount,
        signer_seeds,
    )?;
    let close_instruction = &spl_token::instruction::close_account(
        token_program.key,
        vault.key,
        signer_maker_info.key,
        offer.key,
        &[offer.key, signer_maker_info.key],
    )?;
    let account_infos = &[vault.clone(), signer_maker_info.clone(), offer.clone()];

    invoke_signed_with_bump(
        close_instruction,
        account_infos,
        signer_seeds,
        offer_data.bump as u8,
    )?;

    Ok(())
}
