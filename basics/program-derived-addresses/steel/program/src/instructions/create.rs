use crate::{state::PageVisits, SteelInstruction};
use steel::*;

instruction!(SteelInstruction, CreatePageVisits);

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreatePageVisits {}

impl CreatePageVisits {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo<'_>]) -> ProgramResult {
        let [page_visits_account, user,  payer, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        payer.is_signer()?;
        page_visits_account
            .is_writable()?
            .has_owner(&system_program::ID)? // we check that the account is owned by the system program.
            .has_seeds(
                &[PageVisits::SEED_PREFIX.as_bytes(), user.key.as_ref()],
                program_id,
            )?;
        system_program.is_program(&system_program::ID)?;

        create_account::<PageVisits>(
            page_visits_account,
            system_program,
            payer,
            program_id,
            &[PageVisits::SEED_PREFIX.as_bytes(), user.key.as_ref()],
        )?;

        let page_visit = page_visits_account.as_account_mut::<PageVisits>(program_id)?;

        let page_visits_bump = Pubkey::find_program_address(
            &[PageVisits::SEED_PREFIX.as_bytes(), user.key.as_ref()],
            program_id,
        )
        .1;

        page_visit.page_visits = 0;
        page_visit.bump = page_visits_bump;

        Ok(())
    }
}
