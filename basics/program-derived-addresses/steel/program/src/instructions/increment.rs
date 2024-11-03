use crate::{page_visits::PageVisits, SteelInstruction};
use steel::*;

instruction!(SteelInstruction, IncrementPageVisits);

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct IncrementPageVisits {}

impl IncrementPageVisits {
    pub fn process(accounts: &[AccountInfo<'_>]) -> ProgramResult {
        let [page_visits_account, user] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        page_visits_account
            .is_writable()?
            .has_owner(&crate::ID)?
            .has_seeds(
                &[PageVisits::SEED_PREFIX.as_bytes(), user.key.as_ref()],
                &crate::ID,
            )?;

        let page_visits = page_visits_account.as_account_mut::<PageVisits>(&crate::ID)?;

        page_visits.increment();

        Ok(())
    }
}
