use crate::state::PageVisits;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreatePageVisits<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        init,
        space = 8 + PageVisits::INIT_SPACE,
        payer = payer,
        seeds = [
            PageVisits::SEED_PREFIX,
            payer.key().as_ref(),
        ],
        bump,
    )]
    page_visits: Account<'info, PageVisits>,
    system_program: Program<'info, System>,
}

pub fn handle_create_page_visits(context: Context<CreatePageVisits>) -> Result<()> {
    *context.accounts.page_visits = PageVisits {
        page_visits: 0,
        bump: context.bumps.page_visits,
    };

    Ok(())
}
