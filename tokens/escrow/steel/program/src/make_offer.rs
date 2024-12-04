use escrow_api::prelude::*;
use steel::{transfer as transfer_spl_tokens, *};

pub fn process_make_offer(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = MakeOffer::try_from_bytes(data)?;
    let token_a_offered_amount = u64::from_le_bytes(args.token_a_offered_amount);
    let id = u64::from_le_bytes(args.id);
    let token_b_wanted_amount = u64::from_le_bytes(args.token_b_wanted_amount);

    // Load accounts.
    let [maker_info, token_mint_a_info, token_mint_b_info, maker_token_account_a_info, offer_info, vault_info, system_program, token_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    maker_info.is_signer()?;

    token_mint_a_info
        .has_address(&token_mint_a_info.key)?
        .is_writable()?
        .as_mint()?;

    token_mint_b_info
        .has_address(&token_mint_b_info.key)?
        .is_writable()?
        .as_mint()?;

    maker_token_account_a_info
        .as_associated_token_account(maker_info.key, token_mint_a_info.key)?;

    offer_info.is_empty()?.is_writable()?.has_seeds(
        &[OFFER, maker_info.key.as_ref(), id.to_le_bytes().as_ref()],
        &escrow_api::ID,
    )?;

    vault_info.is_empty()?.is_writable()?;

    // Initialize offer.
    create_account::<Offer>(
        offer_info,
        system_program,
        maker_info,
        &escrow_api::ID,
        &[OFFER, maker_info.key.as_ref(), id.to_le_bytes().as_ref()],
    )?;

    // create valut
    create_associated_token_account(
        maker_info,
        offer_info,
        vault_info,
        token_mint_a_info,
        system_program,
        token_program,
        associated_token_program,
    )?;

    transfer_spl_tokens(
        maker_info,
        maker_token_account_a_info,
        vault_info,
        token_program,
        token_a_offered_amount,
    )?;

    let offer = offer_info.as_account_mut::<Offer>(&escrow_api::ID)?;
    offer.id = id;
    offer.maker = *maker_info.key;
    offer.token_mint_a = *token_mint_a_info.key;
    offer.token_mint_b = *token_mint_b_info.key;
    offer.token_b_wanted_amount = token_b_wanted_amount;
    offer.bump = offer_pda(*maker_info.key, id).1;

    Ok(())
}
