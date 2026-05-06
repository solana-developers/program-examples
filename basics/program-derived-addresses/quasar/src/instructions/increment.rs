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

#[inline(always)]
pub fn handle_increment_page_visits(accounts: &mut IncrementPageVisits) -> Result<(), ProgramError> {
    let current: u64 = accounts.page_visits.page_visits.into();
    accounts.page_visits.page_visits = PodU64::from(current.checked_add(1).unwrap());
    Ok(())
}
