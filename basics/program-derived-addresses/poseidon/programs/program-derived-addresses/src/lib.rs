use anchor_lang::prelude::*;
declare_id!("HcZYDcJ4AF3LgKnDaF6jKyBY7Lf2zggPn4vdGvHBACiW");
#[program]
pub mod program_derived_addresses {
    use super::*;
    pub fn create_page_visits(ctx: Context<CreatePageVisitsContext>) -> Result<()> {
        ctx.accounts.page_visits.page_visits = 0;
        ctx.accounts.page_visits.bump = ctx.bumps.page_visits;
        Ok(())
    }
    pub fn increment_page_visits(ctx: Context<IncrementPageVisitsContext>) -> Result<()> {
        ctx.accounts.page_visits.page_visits = ctx.accounts.page_visits.page_visits + 1;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreatePageVisitsContext<'info> {
    #[account(
        init,
        payer = user,
        space = 13,
        seeds = [b"page_visits", user.key().as_ref()],
        bump,
    )]
    pub page_visits: Account<'info, PageVisits>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct IncrementPageVisitsContext<'info> {
    #[account(mut)]
    pub page_visits: Account<'info, PageVisits>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct PageVisits {
    pub page_visits: u32,
    pub bump: u8,
}
