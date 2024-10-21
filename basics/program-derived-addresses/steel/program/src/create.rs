use program_derived_addresses_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_create(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    msg!("Processing Create instruction");

    // parse args
    let args = Create::try_from_bytes(data)?;
    let page_visits = args.page_visits;

    // load accounts
    let [signer_info, user_info, pages_visit_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // get expected pda
    let pda = page_visits_pda(user_info.key);

    // validations
    signer_info.is_signer()?;
    pages_visit_info.is_empty()?.is_writable()?.has_seeds(
        &[SEED, &user_info.key.as_ref()],
        pda.1,
        &program_derived_addresses_api::ID,
    )?;
    system_program.is_program(&system_program::ID)?;

    // create account
    create_account::<PageVisits>(
        pages_visit_info,
        &program_derived_addresses_api::ID,
        &[SEED, user_info.key.as_ref(), &[pda.1]],
        system_program,
        signer_info,
    )?;

    let info = pages_visit_info.to_account_mut::<PageVisits>(&program_derived_addresses_api::ID)?;
    info.page_visits = page_visits.page_visits;
    info.bump = page_visits.bump;

    msg!("page visits: {:?}", u32::from_le_bytes(info.page_visits));

    Ok(())
}
