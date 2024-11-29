use anchor_lang::prelude::*;
declare_id!("AXXZKfVRcFtAkRbk3oZU8rVStmP2nRY54xky1C7Mi6mb");
#[program]
pub mod close_account {
    use super::*;
    pub fn create_user(ctx: Context<CreateUserContext>) -> Result<()> {
        ctx.accounts.user_account.user = ctx.accounts.user.key();
        ctx.accounts.user_account.bump = ctx.bumps.user_account;
        Ok(())
    }
    pub fn close_user(_ctx: Context<CloseUserContext>) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateUserContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 41,
        seeds = [b"user",
        user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, CloseAccountState>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseUserContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"user", user.key().as_ref()], bump, close = user)]
    pub user_account: Account<'info, CloseAccountState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct CloseAccountState {
    pub user: Pubkey,
    pub bump: u8,
}
