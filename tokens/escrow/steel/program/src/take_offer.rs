use escrow_api::prelude::*;
use steel::*;

pub fn process_take_offer(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [taker_info, maker_info, mint_a_info, mint_b_info, taker_token_account_a_info, taker_token_account_b_info, maker_token_account_b_info, offer_info, vault_info, token_program, system_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    taker_info.is_signer()?;
    token_program.is_program(&spl_token::ID)?;
    system_program.is_program(&system_program::ID)?;
    associated_token_program.is_program(&ASSOCIATED_TOKEN_PROGRAM_ID)?;

    let vaul = vault_info.as_associated_token_account(offer_info.key, mint_a_info.key)?;

    // validate mint
    let _mint_a = mint_a_info.as_mint()?;
    let _mint_b = mint_b_info.as_mint()?;

    if taker_token_account_a_info.data_is_empty() {
        create_associated_token_account(
            taker_info,
            taker_info,
            taker_token_account_a_info,
            mint_a_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
    }

    let _taker_token_account_b = taker_token_account_b_info
        .is_writable()?
        .as_associated_token_account(taker_info.key, mint_b_info.key)?;

    if maker_token_account_b_info.data_is_empty() {
        create_associated_token_account(
            taker_info,
            maker_info,
            maker_token_account_b_info,
            mint_b_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
    }

    offer_info.is_writable()?;

    let offer: &mut Offer = offer_info.as_account_mut::<Offer>(&escrow_api::ID)?;
    offer_info.has_seeds(
        &[OFFER_SEED, offer.maker.as_ref(), offer.id.as_ref()],
        &escrow_api::ID,
    )?;

    // transfer wanted token from taker to maker
    let token_b_wanted_amount = u64::from_le_bytes(offer.token_b_wanted_amount);
    transfer(
        taker_info,
        taker_token_account_b_info,
        maker_token_account_b_info,
        token_program,
        token_b_wanted_amount,
    )?;

    // // widthdraw token A from vault
    transfer_signed_with_bump(
        offer_info,
        vault_info,
        taker_token_account_a_info,
        token_program,
        vaul.amount,
        &[OFFER_SEED, offer.maker.as_ref(), offer.id.as_ref()],
        offer.bump,
    )?;

    let seeds = &[
        OFFER_SEED,
        offer.maker.as_ref(),
        offer.id.as_ref(),
        &[offer.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    // close vault account
    solana_program::program::invoke_signed(
        &spl_token::instruction::close_account(
            &spl_token::ID,
            vault_info.key,
            taker_info.key,
            offer_info.key,
            &[&offer_info.key],
        )?,
        &[
            token_program.clone(),
            vault_info.clone(),
            taker_info.clone(),
            offer_info.clone(),
        ],
        signer_seeds,
    )?;

    // close offer account
    offer_info.close(maker_info)?;

    Ok(())
}
