use anchor_lang::prelude::*;
declare_id!("7U4SZvsUMjGYCwnzqGGE9enJactKhVPF2EaE7VHtBLTd");


#[program]
pub mod close_account_program {
    use super::*;

    pub fn create_user(ctx: Context<CreateUserContext>) -> Result<()> {
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
    #[account(
        init,
        payer = user,
        space = 8 + closeAccountState::INIT_SPACE,
        seeds = [b"user",
        user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, closeAccountState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseUserContext<'info> {
    #[account()]
    pub user_account: Account<'info, closeAccountState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
#[derive(InitSpace)] 
pub struct closeAccountState {
    pub user: Pubkey,
    pub bump: u8,
}
