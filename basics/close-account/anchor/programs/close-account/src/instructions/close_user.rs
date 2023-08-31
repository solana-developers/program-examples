use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;

use crate::state::*;

#[derive(Accounts)]
pub struct CloseUserContext<'info> {
    #[account(
        mut,
        seeds = [
            User::PREFIX.as_bytes(),
            user.key().as_ref(),
        ],
        has_one = user,
        bump = user_account.bump
    )]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn close_user(ctx: Context<CloseUserContext>) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let user_account = &mut ctx.accounts.user_account;
    user_account.close(user.to_account_info())?;
    Ok(())
}
