use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
declare_id!("DwKzVHWsZEUKrySDHQwFW1J8nFWGtz7HzjY9FY55PMDS");
#[program]
pub mod transfer_sol {
    use super::*;
    pub fn transfer_sol_with_cpi(
        ctx: Context<TransferSolWithCpiContext>,
        amount: u64,
    ) -> Result<()> {
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
    pub fn transfer_sol_with_program(
        ctx: Context<TransferSolWithProgramContext>,
        amount: u64,
    ) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct TransferSolWithCpiContext<'info> {
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct TransferSolWithProgramContext<'info> {
    #[account(mut)]
    /// CHECK: This acc is safe
    pub payer: UncheckedAccount<'info>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
