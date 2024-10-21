use program_derived_addresses_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_increment(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    msg!("Processing Create instruction");

    // load accounts.
    let [page_visit_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let info = page_visit_info.to_account_mut::<PageVisits>(&program_derived_addresses_api::ID)?;

    // increment page visits
    info.increment_visits();

    Ok(())
}
