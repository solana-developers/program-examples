use anchor_lang::prelude::*;

declare_id!("6NUjZjUaC62bA6En3Cco9bhW4oKp7nqD6pBjmfcXKTyV");

#[program]
pub mod rent_example {
    use super::*;

    pub fn check_rent_exemption(ctx: Context<CheckRentExemption>) -> Result<()> {
        let account = &ctx.accounts.user_account;
        let lamports = account.to_account_info().lamports();
        let rent = Rent::get()?.minimum_balance(account.to_account_info().data_len());
        
        if lamports >= rent * 2 {
            msg!("Account is rent-exempt.");
        } else {
            msg!("Account is NOT rent-exempt.");
        }
        Ok(())
    }
}


#[derive(Accounts)]
pub struct CheckRentExemption<'info> {
    #[account(mut)]
    pub user_account: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
}
