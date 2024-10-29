use anchor_lang::prelude::*;
declare_id!("DDhy3V9AQE4wrJ3DXC5Yop2p56J6TiBRaz4zjnYnK8ao");
#[program]
pub mod close_account {
    use super::*;
    pub fn create_user(ctx: Context<CreateUserContext>, name: String) -> Result<()> {
        ctx.accounts.user_state.user_bump = ctx.bumps.user_state;
        ctx.accounts.user_state.user = ctx.accounts.user.key();
        ctx.accounts.user_state.name = name;
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
        space = 95,
        seeds = [b"USER",
        user.key().as_ref()],
        bump,
    )]
    pub user_state: Account<'info, UserState>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseUserContext<'info> {
    #[account(mut, seeds = [b"USER", user.key().as_ref()], bump, close = user)]
    pub user_account: Account<'info, UserState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct UserState {
    pub user_bump: u8,
    pub name: String,
    pub user: Pubkey,
}
