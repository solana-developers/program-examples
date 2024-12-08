use anchor_lang::prelude::*;
declare_id!("HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1");
#[program]
pub mod close_account_program {
    use super::*;
    pub fn create_user(ctx: Context<CreateUserContext>, name: String) -> Result<()> {
        ctx.accounts.user_account.user = ctx.accounts.user.key();
        ctx.accounts.user_account.bump = ctx.bumps.user_account;
        Ok(())
    }
    pub fn close_user(ctx: Context<CloseUserContext>) -> Result<()> {
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
        space = 49,
        seeds = [b"USER",
        user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserState>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseUserContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"USER",
        user.key().as_ref()],
        bump = user_account.bump,
        close = user,
    )]
    pub user_account: Account<'info, UserState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct UserState {
    pub bump: u8,
    pub user: Pubkey,
    pub name: String,
    pub name_size: u64,
}
