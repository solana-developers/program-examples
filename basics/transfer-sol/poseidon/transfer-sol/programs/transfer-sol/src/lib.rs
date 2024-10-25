use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
declare_id!("97hdpUKsrwzwkThN7hiySbssDRujp3kjFnKEpuSD66uk");
#[program]
pub mod transfer_sol {
    use super::*;
    pub fn transfer_sol(
        ctx: Context<TransferSolContext>,
        transfer_amount: u64,
    ) -> Result<()> {
        let transfer_accounts = Transfer {
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_accounts,
        );
        transfer(cpi_ctx, transfer_amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct TransferSolContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
