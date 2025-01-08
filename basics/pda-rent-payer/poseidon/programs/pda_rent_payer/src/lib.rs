use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
declare_id!("BYj8GpV9hpv9PAVdwoWFCTMkysJkk5jstYjuCrw4pxem");
#[program]
pub mod pda_rent_payer {
    use super::*;
    pub fn init_rent_vault(ctx: Context<InitRentVaultContext>) -> Result<()> {
        ctx.accounts.state.owner = ctx.accounts.owner.key();
        ctx.accounts.state.state_bump = ctx.bumps.state;
        ctx.accounts.state.auth_bump = ctx.bumps.auth;
        ctx.accounts.state.vault_bump = ctx.bumps.vault;
        Ok(())
    }
    pub fn deposit_to_rent_vault(
        ctx: Context<DepositToRentVaultContext>,
        amount: u64,
    ) -> Result<()> {
        let transfer_accounts = Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_accounts,
        );
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
    pub fn create_new_account(
        ctx: Context<CreateNewAccountContext>,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.new_account_state.owner = ctx.accounts.owner.key();
        let transfer_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.new_account_state.to_account_info(),
        };
        let seeds = &[
            b"vault",
            ctx.accounts.auth.to_account_info().key.as_ref(),
            &[ctx.accounts.state.vault_bump],
        ];
        let pda_signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            transfer_accounts,
            pda_signer,
        );
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitRentVaultContext<'info> {
    #[account(
        init,
        payer = owner,
        space = 8,
        seeds = [b"vault",
        auth.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, RentVault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 43,
        seeds = [b"state",
        owner.key().as_ref()],
        bump,
    )]
    pub state: Account<'info, RentAccountState>,
    #[account(seeds = [b"auth", state.key().as_ref()], bump)]
    /// CHECK: This acc is safe
    pub auth: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct DepositToRentVaultContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(seeds = [b"auth", state.key().as_ref()], bump = state.auth_bump)]
    /// CHECK: This acc is safe
    pub auth: UncheckedAccount<'info>,
    #[account(seeds = [b"vault", auth.key().as_ref()], bump = state.vault_bump)]
    pub vault: Account<'info, RentVault>,
    #[account(seeds = [b"state", owner.key().as_ref()], bump = state.state_bump)]
    pub state: Account<'info, RentAccountState>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CreateNewAccountContext<'info> {
    #[account(
        init,
        payer = owner,
        space = 41,
        seeds = [b"new_account",
        owner.key().as_ref()],
        bump,
    )]
    pub new_account_state: Account<'info, NewAccountState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(seeds = [b"auth", state.key().as_ref()], bump = state.auth_bump)]
    /// CHECK: This acc is safe
    pub auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"vault", auth.key().as_ref()], bump = state.vault_bump)]
    pub vault: SystemAccount<'info>,
    #[account(seeds = [b"state", owner.key().as_ref()], bump = state.state_bump)]
    pub state: Account<'info, RentAccountState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct RentAccountState {
    pub owner: Pubkey,
    pub state_bump: u8,
    pub auth_bump: u8,
    pub vault_bump: u8,
}
#[account]
pub struct NewAccountState {
    pub owner: Pubkey,
    pub new_account_bump: u8,
}
#[account]
pub struct RentVault {}
