use anchor_lang::prelude::*;
declare_id!("EHjrAJo1Ld77gkq6Pp2ErQHcC6FghT8BEPebNve8bAvj");
#[program]
pub mod rent_program {
    use super::*;
    pub fn create_system_account(
        ctx: Context<CreateSystemAccountContext>,
        id: u64,
        zip_code: u64,
    ) -> Result<()> {
        ctx.accounts.account.account_bump = ctx.bumps.account;
        ctx.accounts.account.owner = ctx.accounts.owner.key();
        ctx.accounts.account.id = id;
        ctx.accounts.account.zip_code = zip_code;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateSystemAccountContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner, space = 57, seeds = [b"account"], bump)]
    pub account: Account<'info, AddressData>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct AddressData {
    pub owner: Pubkey,
    pub id: u64,
    pub zip_code: u64,
    pub account_bump: u8,
}
