use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
declare_id!("Db6bufzTcWMDhiCUAuCv3AqCyeZKV4BSGG9ooCibQjrJ");
#[program]
pub mod pda_rent_payer {
    use super::*;
    pub fn init_rent_vault(ctx: Context<InitRentVaultContext>, fund_lamports: u64) -> Result<()> {
        let transfer_accounts = Transfer {
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.rent_vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_accounts,
        );
        transfer(cpi_ctx, fund_lamports)?;
        Ok(())
    }
    pub fn create_new_account(ctx: Context<CreateNewAccountContext>, amount: u64) -> Result<()> {
        let transfer_accounts = Transfer {
            from: ctx.accounts.rent_vault.to_account_info(),
            to: ctx.accounts.new_account.to_account_info(),
        };
        let signer_seeds: &[&[&[u8]]; 1] = &[&[b"rent_vault", &[ctx.bumps.rent_vault]]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitRentVaultContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"rent_vault"], bump)]
    pub rent_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CreateNewAccountContext<'info> {
    #[account(mut, seeds = [b"rent_vault"], bump)]
    pub rent_vault: SystemAccount<'info>,
    #[account(mut)]
    pub new_account: Signer<'info>,
    pub system_program: Program<'info, System>,
}
