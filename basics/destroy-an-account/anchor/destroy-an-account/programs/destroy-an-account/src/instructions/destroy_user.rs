use crate::state::user::User;
use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;

#[derive(Accounts)]
pub struct DestroyUserContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    mut,
    seeds=[User::PREFIX.as_bytes(), user_account.user.key().as_ref()],
    has_one=user,
    bump=user_account.bump
    )]
    pub user_account: Box<Account<'info, User>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn destroy_user(ctx: Context<DestroyUserContext>) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let user_account = &mut ctx.accounts.user_account;
    user_account.close(user.to_account_info())?;
    Ok(())
}
