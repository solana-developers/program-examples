use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod transfer_sol_program {
    use super::*;

    pub fn transfer_sol(ctx: Context<TransferSolContext>, amount: u64) -> Result<()> {
        let transfer_accounts = Transfer {
            from: ctx.accounts.sender.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.key(),
            transfer_accounts,
        );
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferSolContext<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    /// CHECK: This account is not read or written
    pub recipient: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
