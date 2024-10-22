use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
declare_id!("BYj8GpV9hpv9PAVdwoWFCTMkysJkk5jstYjuCrw4pxem");
#[program]
pub mod pda_rent_payer {
    use super::*;
    pub fn init_rent_vault(
        ctx: Context<InitRentVaultContext>,
        fund_lamports: u64,
    ) -> Result<()> {
        ctx.accounts.rent_vault.rent_vault_bump = ctx.bumps.rent_vault;
        let transfer_accounts = Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.rent_vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_accounts,
        );
        transfer(cpi_ctx, fund_lamports)?;
        Ok(())
    }
    pub fn create_new_account(
        ctx: Context<CreateNewAccountContext>,
        transfer_amount: u64,
    ) -> Result<()> {
        ctx.accounts.new_account.owner = ctx.accounts.owner.key();
        ctx.accounts.new_account.account_bump = ctx.bumps.new_account;
        let transfer_accounts = Transfer {
            from: ctx.accounts.rent_vault.to_account_info(),
            to: ctx.accounts.new_account.to_account_info(),
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
pub struct InitRentVaultContext<'info> {
    #[account(
        init,
        payer = owner,
        space = 9,
        seeds = [b"rent_vault",
        owner.key().as_ref()],
        bump,
    )]
    pub rent_vault: Account<'info, RentState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CreateNewAccountContext<'info> {
    #[account()]
    pub rent_vault: Account<'info, RentState>,
    #[account(
        init,
        payer = owner,
        space = 41,
        seeds = [b"account",
        owner.key().as_ref()],
        bump,
    )]
    pub new_account: Account<'info, AccountState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct AccountState {
    pub owner: Pubkey,
    pub account_bump: u8,
}
#[account]
pub struct RentState {
    pub rent_vault_bump: u8,
}
