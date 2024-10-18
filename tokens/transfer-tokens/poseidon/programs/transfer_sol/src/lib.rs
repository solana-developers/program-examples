use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
declare_id!("HU7QEokj5qUUV5ryZYL7EhsqiAkxJyMVXc3DesKyCqtF");
#[program]
pub mod transfer_sol {
    use super::*;
    pub fn transfer_with_program(
        ctx: Context<TransferWithProgramContext>,
        amount: u64,
    ) -> Result<()> {
        let transfer_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
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
pub struct TransferWithProgramContext<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
