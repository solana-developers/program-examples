use crate::state::user::User;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateUserArgs {
    pub name: String,
}

#[derive(Accounts)]
#[instruction(args: CreateUserArgs)]
pub struct CreateUserContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    init,
    seeds=[User::PREFIX.as_bytes(), user.key().as_ref()],
    payer=user,
    space=User::SIZE,
    bump
    )]
    pub user_account: Box<Account<'info, User>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_user(ctx: Context<CreateUserContext>, args: CreateUserArgs) -> Result<()> {
    let user = &ctx.accounts.user;
    let user_account = &mut ctx.accounts.user_account;

    user_account.bump = *ctx.bumps.get("user_account").unwrap();
    user_account.user = user.key();
    user_account.name = args.name;

    Ok(())
}
