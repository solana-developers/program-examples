use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
declare_id!("7ogRvKFEYpbceu1CorL3V9jFdwPKkWdpUgUqNdsVswbG");
#[program]
pub mod poseidon {
    use super::*;
    pub fn transfer_sol(ctx: Context<TransferSolContext>, amount: u64) -> Result<()> {
        let transfer_accounts = Transfer {
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
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
pub struct TransferSolContext<'info> {
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
