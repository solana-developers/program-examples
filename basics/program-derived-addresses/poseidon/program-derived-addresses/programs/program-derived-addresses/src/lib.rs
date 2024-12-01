use anchor_lang::prelude::*;
declare_id!("F1TXTbegcoBMBNFJPp8QVx9eMnh62qDSNMWU2FVWqV5i");
#[program]
pub mod program_derived_addresses {
    use super::*;
    pub fn create_page_visits(ctx: Context<CreatePageVisitsContext>) -> Result<()> {
        ctx.accounts.page_visits.bump = ctx.bumps.page_visits;
        ctx.accounts.page_visits.page_visits = 0;
        Ok(())
    }
    pub fn increment(ctx: Context<IncrementContext>) -> Result<()> {
        ctx.accounts.page_visits.page_visits += 1;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreatePageVisitsContext<'info> {
    #[account(
        init,
        payer = payer,
        space = 13,
        seeds = [b"page_visits",
        payer.key().as_ref()],
        bump,
    )]
    pub page_visits: Account<'info, PageVisit>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct IncrementContext<'info> {
    #[account(mut, seeds = [b"page_visits", user.key().as_ref()], bump)]
    pub page_visits: Account<'info, PageVisit>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct PageVisit {
    pub page_visits: u32,
    pub bump: u8,
}
