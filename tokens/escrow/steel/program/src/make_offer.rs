use escrow_api::prelude::*;
use steel::*;

pub fn process_make_offer(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = MakeOffer::try_from_bytes(data)?;
    let id = u64::from_le_bytes(args.id);
    let token_a_offered_amount = u64::from_le_bytes(args.token_a_offered_amount);

    // Load accounts.
    let [maker_info, mint_a_info, mint_b_info, maker_token_account_a_info, offer_info, vault_info, token_program, system_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    maker_info.is_signer()?;
    token_program.is_program(&spl_token::ID)?;
    system_program.is_program(&system_program::ID)?;
    associated_token_program.is_program(&ASSOCIATED_TOKEN_PROGRAM_ID)?;

    offer_info.is_empty()?.is_writable()?.has_seeds(
        &[
            OFFER_SEED,
            maker_info.key.as_ref(),
            id.to_le_bytes().as_ref(),
        ],
        &escrow_api::ID,
    )?;

    vault_info.is_empty()?.is_writable()?;

    // Create associated token account for vault
    create_associated_token_account(
        maker_info,
        offer_info,
        vault_info,
        mint_a_info,
        system_program,
        token_program,
        associated_token_program,
    )?;

    // Call CPI to create account offer
    create_account::<Offer>(
        offer_info,
        system_program,
        maker_info,
        &escrow_api::ID,
        &[
            OFFER_SEED,
            maker_info.key.as_ref(),
            id.to_le_bytes().as_ref(),
        ],
    )?;

    let _mint_a = mint_a_info.as_mint()?;
    let _mint_b = mint_b_info.as_mint()?;

    // transfer token A to vault
    transfer(
        maker_info,
        maker_token_account_a_info,
        vault_info,
        token_program,
        token_a_offered_amount,
    )?;

    let offer: &mut Offer = offer_info.as_account_mut::<Offer>(&escrow_api::ID)?;

    // Update state
    offer.id = args.id;
    offer.maker = *maker_info.key;
    offer.token_mint_a = *mint_a_info.key;
    offer.token_mint_b = *mint_b_info.key;
    offer.token_b_wanted_amount = args.token_b_wanted_amount;
    offer.bump = offer_pda(*maker_info.key, id).1;

    Ok(())
}
