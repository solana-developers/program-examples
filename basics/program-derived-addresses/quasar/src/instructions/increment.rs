use {
    crate::state::PageVisits,
    quasar_lang::prelude::*,
};

/// Accounts for incrementing page visits.
/// The user account is needed to derive the PDA seeds for validation.
#[derive(Accounts)]
pub struct IncrementPageVisits<'info> {
    pub user: &'info UncheckedAccount,
    #[account(mut)]
    pub page_visits: &'info mut Account<PageVisits>,
}

impl<'info> IncrementPageVisits<'info> {
    #[inline(always)]
    pub fn increment_page_visits(&mut self) -> Result<(), ProgramError> {
        let current: u64 = self.page_visits.page_visits.into();
        self.page_visits.page_visits = PodU64::from(current.checked_add(1).unwrap());
        Ok(())
    }
}
