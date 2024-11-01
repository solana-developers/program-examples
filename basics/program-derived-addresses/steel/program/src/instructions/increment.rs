use crate::{page_visits::PageVisits, SteelInstruction};
use steel::*;

instruction!(SteelInstruction, IncrementPageVisits);

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct IncrementPageVisits {}

impl IncrementPageVisits {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo<'_>],
    ) -> ProgramResult {
        let [page_visits_account, user] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        page_visits_account
            .is_writable()?
            .has_owner(program_id)? 
            .has_seeds(
                &[PageVisits::SEED_PREFIX.as_bytes(), user.key.as_ref()],
                program_id,
            )?;

        let page_visits = page_visits_account.as_account_mut::<PageVisits>(program_id)?;

        page_visits.increment();

        Ok(())
    }
}
