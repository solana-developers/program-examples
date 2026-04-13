use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseUserContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"USER",
            user.key().as_ref(),
        ],
        bump = user_account.bump,
        close = user, // close account and return lamports to user
    )]
    pub user_account: Account<'info, UserState>,
}

pub fn handle_close_user(_context: Context<CloseUserContext>) -> Result<()> {
    Ok(())
}
