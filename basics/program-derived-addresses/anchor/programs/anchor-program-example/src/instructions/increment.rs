use crate::state::PageVisits;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct IncrementPageVisits<'info> {
    user: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [
            PageVisits::SEED_PREFIX,
            user.key().as_ref(),
        ],
        bump = page_visits.bump,
    )]
    page_visits: Account<'info, PageVisits>,
}

pub fn handle_increment_page_visits(context: Context<IncrementPageVisits>) -> Result<()> {
    let page_visits = &mut context.accounts.page_visits;
    page_visits.increment();
    Ok(())
}
