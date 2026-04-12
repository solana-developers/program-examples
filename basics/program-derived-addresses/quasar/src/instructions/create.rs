use {
    crate::state::PageVisits,
    quasar_lang::prelude::*,
};

/// Accounts for creating a new page visits counter.
/// The counter is derived as a PDA from ["page_visits", payer] seeds.
#[derive(Accounts)]
pub struct CreatePageVisits<'info> {
    #[account(mut)]
    pub payer: &'info mut Signer,
    #[account(mut, init, payer = payer, seeds = [b"page_visits", payer], bump)]
    pub page_visits: &'info mut Account<PageVisits>,
    pub system_program: &'info Program<System>,
}

#[inline(always)]
pub fn handle_create_page_visits(accounts: &mut CreatePageVisits) -> Result<(), ProgramError> {
    accounts.page_visits.set_inner(0u64);
    Ok(())
}
