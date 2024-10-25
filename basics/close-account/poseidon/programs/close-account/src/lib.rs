use anchor_lang::prelude::*;
declare_id!("JxT1KDtKktz8Hv6GMjMkL2FNu7BmrD17brHtdERAunH");
#[program]
pub mod close_account {
    use super::*;
    pub fn create_user_account(ctx: Context<CreateUserAccountContext>) -> Result<()> {
        ctx.accounts.user_account.bump = ctx.bumps.user_account;
        ctx.accounts.user_account.user = ctx.accounts.user.key();
        Ok(())
    }
    pub fn close_user_account(ctx: Context<CloseUserAccountContext>) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateUserAccountContext<'info> {
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
    pub user_account: Account<'info, AccountState>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseUserAccountContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"user", user.key().as_ref()], bump, close = user)]
    pub user_account: Account<'info, AccountState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct AccountState {
    pub user: Pubkey,
    pub bump: u8,
}
