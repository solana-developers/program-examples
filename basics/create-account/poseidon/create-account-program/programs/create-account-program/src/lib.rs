use anchor_lang::prelude::*;
declare_id!("Da8ZQ19giGFQgPmxS2vAKkNM5oxsypxWSRN61FeXWAxi");
#[program]
pub mod create_account_program {
    use super::*;
    pub fn create_system_account(
        ctx: Context<CreateSystemAccountContext>,
    ) -> Result<()> {
        ctx.accounts.account.owner = ctx.accounts.owner.key();
        ctx.accounts.account.account_bump = ctx.bumps.account;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateSystemAccountContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner, space = 41, seeds = [b"account"], bump)]
    pub account: Account<'info, AccountState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct AccountState {
    pub owner: Pubkey,
    pub account_bump: u8,
}
