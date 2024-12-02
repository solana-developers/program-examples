use anchor_lang::prelude::*;
declare_id!("8MWRHcfRvyUJpou8nD5oG7DmZ2Bmg99qBP8q5fZ5xJpg");
#[program]
pub mod checking_accounts {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>, data: u64) -> Result<()> {
        ctx.accounts.user_account.user_data = data;
        ctx.accounts.user_account.authority = ctx.accounts.payer.key();
        Ok(())
    }
    pub fn update(ctx: Context<UpdateContext>, new_data: u64) -> Result<()> {
        ctx.accounts.user_account.user_data = new_data;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(init, payer = payer, space = 48, seeds = [b"program"], bump)]
    pub user_account: Account<'info, UserAccountState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct UpdateContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"program"], has_one = authority, bump)]
    pub user_account: Account<'info, UserAccountState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct UserAccountState {
    pub user_data: u64,
    pub authority: Pubkey,
}
