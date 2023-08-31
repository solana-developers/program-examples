use anchor_lang::prelude::*;

use crate::state::PageVisits;

pub fn create_page_visits(ctx: Context<CreatePageVisits>) -> Result<()> {
    ctx.accounts.page_visits.set_inner(PageVisits::new(
        0,
        *ctx.bumps
            .get(PageVisits::SEED_PREFIX)
            .expect("Bump not found."),
    ));
    Ok(())
}

#[derive(Accounts)]
pub struct CreatePageVisits<'info> {
    #[account(
        init,
        space = PageVisits::ACCOUNT_SPACE,
        payer = payer,
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
