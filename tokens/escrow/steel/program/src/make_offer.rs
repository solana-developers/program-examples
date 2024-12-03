use escrow_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_make_offer(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Check number of accounts provided
    if accounts.len() < 9 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    // Parse args.
    let args = MakeOffer::try_from_bytes(data)?;
    let token_a_offered = u64::from_le_bytes(args.token_a_offered_amount);
    let token_b_wanted_amount = u64::from_le_bytes(args.token_b_wanted_amount);
    let id = u64::from_le_bytes(args.id);
    let offer_bump = u8::from_le_bytes(args.bump);

    // Load accounts.
    let [signer_maker_info, token_mint_a, token_mint_b, maker_token_account_a, offer, vault, token_program, associated_token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    //Validating Accounts
    signer_maker_info.is_signer()?;
    token_mint_a.as_mint()?;
    token_mint_b.as_mint()?;
    maker_token_account_a.as_associated_token_account(signer_maker_info.key, token_mint_a.key)?;
    vault.is_empty()?.is_writable()?;
    offer.is_empty()?.is_writable()?.has_seeds(
        &[
            OFFER,
            signer_maker_info.key.as_ref(),
            id.to_le_bytes().as_ref(),
        ],
        &escrow_api::ID,
    )?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    //Create offer account
    create_account::<Offer>(
        offer,
        system_program,
        signer_maker_info,
        &escrow_api::ID,
        &[
            OFFER,
            signer_maker_info.key.as_ref(),
            id.to_le_bytes().as_ref(),
        ],
    )?;

    create_associated_token_account(
        signer_maker_info,
        offer,
        vault,
        token_mint_a,
        system_program,
        token_program,
        associated_token_program,
    )?;

    msg!(
        "ATA's amount balance:{:?}, token offered:{:?}",
        maker_token_account_a
            .as_associated_token_account(signer_maker_info.key, token_mint_a.key)?
            .amount,
        token_a_offered
    );
    // Move the tokens from the maker's ATA to the vault
    transfer(
        signer_maker_info,
        maker_token_account_a,
        vault,
        token_program,
        token_a_offered,
    )?;

    // Save the details of the offer to the offer account
    let offer_data: &mut Offer = offer.as_account_mut::<Offer>(&escrow_api::ID)?;

    offer_data.id = id;
    offer_data.maker = *signer_maker_info.key;
    offer_data.token_mint_a = *token_mint_a.key;
    offer_data.token_mint_b = *token_mint_b.key;
    offer_data.token_b_wanted_amount = token_b_wanted_amount;
    offer_data.bump = offer_bump as u64;

    Ok(())
}
