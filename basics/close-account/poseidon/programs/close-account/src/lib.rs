use anchor_lang::prelude::*;
declare_id!("4So9Jbx672BRL9RvfB8Sux2NMVX5QJRnhmdWyij3kkFg");
#[program]
pub mod close_account {
    use super::*;
    pub fn initalize(ctx: Context<InitalizeContext>, data: u8) -> Result<()> {
        ctx.accounts.state.some_data = data;
        Ok(())
    }
    pub fn close(ctx: Context<CloseContext>) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitalizeContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 9, seeds = [b"account"], bump)]
    pub state: Account<'info, AccountState>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"account"], bump, close = user)]
    pub state: Account<'info, AccountState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct AccountState {
    pub some_data: u8,
}
