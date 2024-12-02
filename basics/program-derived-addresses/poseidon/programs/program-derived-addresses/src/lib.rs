use anchor_lang::prelude::*;
declare_id!("GBQw9SP64U2WYhRwwWCQswd4KPcK19cSSw7BvdxK9hyG");
#[program]
pub mod program_derived_addresses {
    use super::*;
    pub fn create_page_visits(
        ctx: Context<CreatePageVisitsContext>,
        seed: u64,
    ) -> Result<()> {
        ctx.accounts.page_visits.page_visits = 0;
        ctx.accounts.page_visits.seed = seed;
        Ok(())
    }
    pub fn increment_page_visits(
        ctx: Context<IncrementPageVisitsContext>,
    ) -> Result<()> {
        ctx.accounts.page_visits.page_visits = ctx.accounts.page_visits.page_visits + 1;
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct CreatePageVisitsContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 20,
        seeds = [seed.to_le_bytes().as_ref(),
        payer.key().as_ref()],
        bump,
    )]
    pub page_visits: Account<'info, PageVisits>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct IncrementPageVisitsContext<'info> {
    #[account(
        mut,
        seeds = [page_visits.seed.to_le_bytes().as_ref(),
        payer.key().as_ref()],
        bump,
    )]
    pub page_visits: Account<'info, PageVisits>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct PageVisits {
    pub seed: u64,
    pub page_visits: u32,
}
