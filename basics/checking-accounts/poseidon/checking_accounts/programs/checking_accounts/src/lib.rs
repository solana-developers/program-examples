use anchor_lang::prelude::*;
declare_id!("DqZo8ioCBtRiFibxQeWrHUtE8ZES5ETA6Uq3hgAYWsUD");
#[program]
pub mod check_accounts_program {
    use super::*;
    pub fn check_accounts(ctx: Context<CheckAccountsContext>) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CheckAccountsContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account()]
    /// CHECK: This acc is safe
    pub account_to_create: UncheckedAccount<'info>,
    #[account()]
    /// CHECK: This acc is safe
    pub account_to_change: UncheckedAccount<'info>,
}
