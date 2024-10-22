use anchor_lang::prelude::*;
declare_id!("9M9xaYvQeFcBf2eHsJJPJWaQB34yUHKgcumskCCKM875");
#[program]
pub mod rent_program {
    use super::*;
    pub fn create_system_account(
        ctx: Context<CreateSystemAccountContext>,
    ) -> Result<()> {
        ctx.accounts.account.owner = ctx.accounts.owner.key();
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateSystemAccountContext<'info> {
    #[account(init, payer = owner, space = 44, seeds = [b"account"], bump)]
    pub account: Account<'info, AccountState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct AccountState {
    pub owner: Pubkey,
    pub space: u32,
}
