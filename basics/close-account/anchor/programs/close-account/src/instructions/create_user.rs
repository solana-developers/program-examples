use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateUserContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = UserState::INIT_SPACE,
        seeds = [
            b"USER",
            user.key().as_ref(),
        ],
        bump
    )]
    pub user_account: Account<'info, UserState>,
    pub system_program: Program<'info, System>,
}

pub fn create_user(ctx: Context<CreateUserContext>, name: String) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;

    user_account.bump = *ctx.bumps.get("user_account").unwrap();
    user_account.user = ctx.accounts.user.key();
    user_account.name = name;

    Ok(())
}
