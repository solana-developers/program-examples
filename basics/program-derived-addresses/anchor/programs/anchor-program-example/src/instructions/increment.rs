use anchor_lang::prelude::*;

use crate::state::PageVisits;

pub fn increment_page_visits(ctx: Context<IncrementPageVisits>) -> Result<()> {
    let page_visits = &mut ctx.accounts.page_visits;
    page_visits.increment();
    Ok(())
}

#[derive(Accounts)]
pub struct IncrementPageVisits<'info> {
    #[account(
        mut,
        seeds = [
            PageVisits::SEED_PREFIX.as_bytes(),
            user.key().as_ref(),
        ],
        bump,
    )]
    page_visits: Account<'info, PageVisits>,
    user: SystemAccount<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}
