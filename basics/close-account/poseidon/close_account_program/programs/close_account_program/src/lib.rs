use anchor_lang::prelude::*;
declare_id!("2q4uoWExFAbZjeDe4n3miipsT9bX9vLnkSetCfZYF2VT");
#[program]
pub mod close_account {
    use super::*;
    pub fn create_user(ctx: Context<CreateUserContext>, name: u8) -> Result<()> {
        ctx.accounts.user_account.user_bump = ctx.bumps.user_account;
        ctx.accounts.user_account.user = ctx.accounts.user.key();
        ctx.accounts.user_account.name = name;
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
        space = 42,
        seeds = [b"USER",
        user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseUserContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"USER", user.key().as_ref()], bump, close = user)]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct UserAccount {
    pub user_bump: u8,
    pub name: u8,
    pub user: Pubkey,
}
