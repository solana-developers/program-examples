use anchor_lang::prelude::*;

use crate::state::{Market, UserAccount, USER_ACCOUNT_SEED};

pub fn create_user_account(context: Context<CreateUserAccountAccountConstraints>) -> Result<()> {
    let user_account = &mut context.accounts.user_account;
    user_account.market = context.accounts.market.key();
    user_account.owner = context.accounts.owner.key();
    user_account.unsettled_base = 0;
    user_account.unsettled_quote = 0;
    user_account.open_orders = Vec::new();
    user_account.bump = context.bumps.user_account;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateUserAccountAccountConstraints<'info> {
    #[account(
        init,
        payer = owner,
        space = UserAccount::DISCRIMINATOR.len() + UserAccount::INIT_SPACE,
        seeds = [USER_ACCOUNT_SEED, market.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub market: Account<'info, Market>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}
