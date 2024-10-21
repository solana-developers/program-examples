use anchor_lang::prelude::*;
declare_id!("BW3PD9JUZzMbSEGmZZBRncwMqPgyUpsY1yYbWMDV7aRf");
#[program]
pub mod program_derived_addresses {
    use super::*;
    pub fn create_page_visit(ctx: Context<CreatePageVisitContext>) -> Result<()> {
        ctx.accounts.state.bump = ctx.bumps.state;
        ctx.accounts.state.page_visits = 0;
        Ok(())
    }
    pub fn increment(ctx: Context<IncrementContext>) -> Result<()> {
        ctx.accounts.state.page_visits = ctx.accounts.state.page_visits + 1;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreatePageVisitContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 13,
        seeds = [b"page_visits",
        payer.key().as_ref()],
        bump,
    )]
    pub state: Account<'info, PageVisit>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct IncrementContext<'info> {
    #[account(mut, seeds = [b"page_visits", user.key().as_ref()], bump)]
    pub state: Account<'info, PageVisit>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct PageVisit {
    pub page_visits: u32,
    pub bump: u8,
}
