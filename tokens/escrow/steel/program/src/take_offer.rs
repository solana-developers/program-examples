use escrow_api::prelude::*;
use steel::{transfer as transfer_spl_tokens, *};

pub fn process_take_offer(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [taker_info, maker_info, token_mint_a_info, token_mint_b_info, taker_token_account_a_info, taker_token_account_b_info, maker_token_account_b_info, offer_info, vault_info, system_program, token_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    taker_info.is_signer()?;
    maker_info.is_writable()?;

    let token_mint_a = token_mint_a_info
        .has_address(&token_mint_a_info.key)?
        .is_writable()?
        .as_mint()?;

    token_mint_b_info
        .has_address(&token_mint_b_info.key)?
        .is_writable()?
        .as_mint()?;

    taker_token_account_a_info.is_empty()?.is_writable()?;

    // create taker token account a
    create_associated_token_account(
        taker_info,
        taker_info,
        taker_token_account_a_info,
        token_mint_a_info,
        system_program,
        token_program,
        associated_token_program,
    )?;

    taker_token_account_b_info
        .as_associated_token_account(taker_info.key, token_mint_b_info.key)?;

    maker_token_account_b_info.is_empty()?.is_writable()?;

    // create maker token account b
    create_associated_token_account(
        taker_info,
        maker_info,
        maker_token_account_b_info,
        token_mint_b_info,
        system_program,
        token_program,
        associated_token_program,
    )?;

    let offer = offer_info.as_account_mut::<Offer>(&escrow_api::ID)?;
    offer_info
        .is_writable()?
        .has_seeds(
            &[
                OFFER,
                maker_info.key.as_ref(),
                offer.id.to_le_bytes().as_ref(),
            ],
            &escrow_api::ID,
        )?
        .is_type::<Offer>(&escrow_api::ID)?;

    let vault = vault_info.as_associated_token_account(offer_info.key, token_mint_a_info.key)?;

    transfer_spl_tokens(
        taker_info,
        taker_token_account_b_info,
        maker_token_account_b_info,
        token_program,
        offer.clone().token_b_wanted_amount,
    )?;

    solana_program::program::invoke_signed(
        &spl_token::instruction::transfer_checked(
            token_program.key,
            vault_info.key,
            token_mint_a_info.key,
            taker_token_account_a_info.key,
            offer_info.key,
            &[offer_info.key],
            vault.amount,
            token_mint_a.decimals,
        )?,
        &[
            vault_info.clone(),
            token_mint_a_info.clone(),
            taker_token_account_a_info.clone(),
            offer_info.clone(),
        ],
        &[&[
            OFFER,
            maker_info.key.as_ref(),
            offer.id.to_le_bytes().as_ref(),
            &[offer.bump],
        ]],
    )?;

    // close vault account
    solana_program::program::invoke_signed(
        &spl_token::instruction::close_account(
            &spl_token::ID,
            vault_info.key,
            taker_info.key,
            offer_info.key,
            &[&offer_info.key],
        )?,
        &[vault_info.clone(), taker_info.clone(), offer_info.clone()],
        &[&[
            OFFER,
            maker_info.key.as_ref(),
            offer.id.to_le_bytes().as_ref(),
            &[offer.bump],
        ]],
    )?;

    // close offer account.
    close_account(offer_info, maker_info)?;

    Ok(())
}
