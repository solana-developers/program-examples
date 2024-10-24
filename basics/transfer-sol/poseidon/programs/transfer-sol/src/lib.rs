use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
declare_id!("7VjyAirb4LLbGGTBqzCuYqeirue9S9Zj2fDfUYVU4YdA");


#[program]
pub mod transfer_sol_program {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>, amount: u64) -> Result<()> {
        let transfer_accounts = Transfer {
            from: ctx.accounts.sender.to_account_info(),
            to: ctx.accounts.receiver.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_accounts,
        );
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
